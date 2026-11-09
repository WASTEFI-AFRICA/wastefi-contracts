#!/usr/bin/env python3
"""Prepare a payment contract that derives payouts on-chain, without switching the app."""

import argparse
import fcntl
import hashlib
import importlib.util
from pathlib import Path
import re
import secrets
import subprocess
import sys
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parent.parent
# The point verification rollout already holds the journal, deployment and idempotent
# wiring helpers; this rollout changes one contract and reuses them unchanged.
_SPEC = importlib.util.spec_from_file_location(
    "point_rollout", Path(__file__).resolve().parent / "rollout-point-verification-testnet.py"
)
shared = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(shared)

require, load, save = shared.require, shared.load, shared.save
CONTRACT = "payment_distribution"
NAMES = (
    "waste_token", "collector_registry", "collection_point", "material_pricing",
    "reputation", "waste_transaction", CONTRACT,
)


class DerivedPaymentRollout(shared.Rollout):
    def run(self):
        base = load(self.base)
        require(base.get("network") == "testnet", "Only a testnet base manifest is accepted.")
        old = base["contracts"]
        require(set(old) == set(NAMES), "Base manifest must contain all seven contracts.")
        require(all(re.fullmatch(r"C[A-Z2-7]{55}", v) for v in old.values()), "Invalid contract address.")
        admin = shared.cli("keys", "address", self.deployer)
        require(admin == base["deployer"], "CLI identity does not match the base deployment admin.")

        # Everything this rollout reads or extends must be healthy before any write.
        for name in ("waste_token", "waste_transaction", CONTRACT):
            print(f"Checking existing {name}", flush=True)
            self.expect(old[name], "admin", admin)
            self.expect(old[name], "is_paused", False)
        self.expect(old[CONTRACT], "get_token_contract", old["waste_token"])
        self.expect(old["waste_token"], "is_minter", True, "--minter", old[CONTRACT])

        wasm = self.wasm_dir / f"{CONTRACT}.wasm"
        require(wasm.is_file(), "Build payment_distribution first.")
        digest = hashlib.sha256(wasm.read_bytes()).hexdigest()
        if self.output.exists():
            journal = load(self.output)
            require(journal.get("previous_contracts") == old, "Journal belongs to a different base deployment.")
            require(journal.get("deployer") == admin and journal.get("network") == "testnet", "Journal identity/network mismatch.")
            require(journal.get("wasm_sha256") == {CONTRACT: digest}, "WASM changed since the rollout started; stop for review.")
            require(
                all(journal["contracts"][n] == old[n] for n in NAMES if n != CONTRACT),
                "Journal replaced a preserved contract.",
            )
        else:
            salt = secrets.token_hex(32)
            journal = {
                "network": "testnet", "rollout_state": "preparing",
                "deployment_date": datetime.now(timezone.utc).isoformat(),
                "source_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                "deployer": admin, "previous_contracts": old, "contracts": dict(old),
                "wasm_sha256": {CONTRACT: digest}, "salts": {CONTRACT: salt},
            }
            # Capture changes to deployed Rust source, but ignore rollout docs/tooling.
            if subprocess.run(["git", "diff", "--quiet", "HEAD", "--", "contracts", "Cargo.toml", "Cargo.lock"], cwd=ROOT).returncode:
                journal["source_commit"] += "+uncommitted"
            journal["contracts"][CONTRACT] = shared.cli(
                "contract", "id", "wasm", "--salt", salt, "--source-account", self.deployer, *shared.network_args(),
            ).strip('"')
            # Saved before any deployment, so an interrupted run finds the same address.
            save(self.output, journal)

        address = journal["contracts"][CONTRACT]
        derived = shared.cli(
            "contract", "id", "wasm", "--salt", journal["salts"][CONTRACT],
            "--source-account", self.deployer, *shared.network_args(),
        ).strip('"')
        require(address == derived and address not in old.values(), "Journal salt/address mismatch.")

        print(f"Preparing {CONTRACT}: {address}", flush=True)
        self.deploy(CONTRACT, journal)
        self.ensure(address, "admin", admin, "initialize", write_args=("--admin", admin, "--token_contract", old["waste_token"]))
        self.expect(address, "is_paused", False)
        self.expect(address, "get_token_contract", old["waste_token"])

        print("Binding the transaction contract and approving minting; old roles stay.", flush=True)
        # Set once on-chain: a different existing binding fails here and stops the rollout.
        self.ensure(
            address, "get_transaction_contract", old["waste_transaction"], "set_transaction_contract",
            write_args=("--contract_address", old["waste_transaction"]),
        )
        self.ensure(
            old["waste_token"], "is_minter", True, "add_minter",
            read_args=("--minter", address), write_args=("--minter", address),
        )
        # Collections already pinned to the old payment contract must still be able to finish.
        self.expect(old["waste_token"], "is_minter", True, "--minter", old[CONTRACT])
        journal["rollout_state"] = "ready_for_app_cutover"
        save(self.output, journal)
        print(f"Candidate verified: {self.output}. App configuration has not changed.", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", type=Path, default=ROOT / "deployed_addresses_testnet.json")
    parser.add_argument("--out", type=Path, default=ROOT / "target/derived-payments-testnet.json")
    parser.add_argument("--wasm-dir", type=Path, default=ROOT / "target/wasm32v1-none/release")
    parser.add_argument("--deployer", default="wastefi-deployer")
    parser.add_argument("--apply", action="store_true", help="Deploy the payment contract, wire it and add its minter role on testnet.")
    args = parser.parse_args()
    require(args.base.resolve() != args.out.resolve(), "Output must not overwrite the active manifest.")
    require(args.out.resolve() != (ROOT / "deployed_addresses_testnet.json").resolve(), "Output must not overwrite the active manifest.")
    require(re.fullmatch(r"[A-Za-z0-9_-]{1,64}", args.deployer) and not re.fullmatch(r"S[A-Z2-7]{55}", args.deployer), "Use a CLI identity name, never a secret key.")
    if not args.apply:
        print("Plan only. Build payment_distribution first (scripts/build-wasm.sh payment_distribution).")
        print("Apply adds one contract and one token minter role; it keeps the token, balances and every existing role.")
        print("It writes a candidate manifest, never the active deployment or application configuration.")
        return
    args.out.parent.mkdir(parents=True, exist_ok=True)
    with args.out.with_suffix(args.out.suffix + ".lock").open("a") as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            raise RuntimeError("Another rollout is using this journal.") from None
        DerivedPaymentRollout(args.base, args.out, args.wasm_dir, args.deployer).run()


if __name__ == "__main__":
    try:
        main()
    except RuntimeError as error:
        # RuntimeError messages above are controlled text, never raw CLI output.
        print(f"Rollout stopped: {error} Do not change app pointers.", file=sys.stderr)
        sys.exit(1)
    except (OSError, ValueError, KeyError, TypeError):
        print("Rollout stopped. Check the public manifest and journal; do not change app pointers.", file=sys.stderr)
        sys.exit(1)

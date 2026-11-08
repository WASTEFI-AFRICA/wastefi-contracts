#!/usr/bin/env python3
"""Prepare a testnet point-verification bundle without switching the app."""

import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import secrets
import subprocess
import sys
import tempfile
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parent.parent
RPC = "https://soroban-testnet.stellar.org"
PASSPHRASE = "Test SDF Network ; September 2015"
REPLACE = ("collection_point", "waste_transaction", "payment_distribution")
REUSE = ("waste_token", "collector_registry", "material_pricing", "reputation")


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def load(path):
    return json.loads(path.read_text())


def save(path, value):
    # A killed process must leave either the old journal or the complete new one.
    with tempfile.NamedTemporaryFile(mode="w", dir=path.parent, delete=False) as tmp:
        json.dump(value, tmp, indent=2)
        tmp.write("\n")
        tmp.flush()
        os.fsync(tmp.fileno())
    os.replace(tmp.name, path)


def cli(*args):
    try:
        result = subprocess.run(
            ["stellar", "--quiet", *args], capture_output=True, text=True, timeout=180
        )
    except (OSError, subprocess.TimeoutExpired):
        raise RuntimeError("Stellar CLI unavailable or timed out; retain the journal and rerun.") from None
    # Never copy raw SDK/CLI errors, environment values or signing material to logs.
    require(result.returncode == 0, "Stellar command failed; retain the journal and rerun.")
    return result.stdout.strip()


def network_args():
    return ("--rpc-url", RPC, "--network-passphrase", PASSPHRASE)


class Rollout:
    def __init__(self, base, output, wasm_dir, deployer):
        self.base = base
        self.output = output
        self.wasm_dir = wasm_dir
        self.deployer = deployer

    def call(self, address, method, *args, write=False):
        raw = cli(
            "contract", "invoke", "--id", address,
            "--source-account", self.deployer, *network_args(),
            "--send", "yes" if write else "no", "--", method, *args,
        )
        # CLI 27 prints no stdout for a successful void-returning mutation.
        # Reads still require an explicit JSON result, including Option None.
        if write and not raw:
            return None
        return json.loads(raw)

    def expect(self, address, method, expected, *args):
        require(self.call(address, method, *args) == expected, f"Check failed: {method}.")

    def ensure(self, address, getter, expected, setter, read_args=(), write_args=()):
        try:
            if self.call(address, getter, *read_args) == expected:
                return
        except (RuntimeError, json.JSONDecodeError):
            # Initialization and setters are idempotent or reject repeat calls.
            # An unknown outcome can only retry the same target and value.
            pass
        try:
            self.call(address, setter, *write_args, write=True)
        except (RuntimeError, json.JSONDecodeError):
            # A submission can succeed even if its response is lost.
            pass
        self.expect(address, getter, expected, *read_args)

    def deploy(self, name, journal):
        address = journal["contracts"][name]
        expected_hash = journal["wasm_sha256"][name]

        def check():
            return cli("contract", "info", "hash", "--id", address, *network_args()).strip('"')

        try:
            actual = check()
        except RuntimeError:
            # The ID is deterministic and saved before submission. Even a lost
            # response cannot cause a second deployment at a different address.
            try:
                result = cli(
                    "contract", "deploy", "--wasm", str(self.wasm_dir / f"{name}.wasm"),
                    "--salt", journal["salts"][name], "--optimize=false",
                    "--source-account", self.deployer, *network_args(),
                )
                require(result.strip('"') == address, "Deployment returned an unexpected address.")
            except RuntimeError:
                pass
            actual = check()
        require(actual == expected_hash, f"Unexpected deployed WASM for {name}; stop for review.")

    def run(self):
        base = load(self.base)
        require(base.get("network") == "testnet", "Only a testnet base manifest is accepted.")
        old = base["contracts"]
        require(set(old) == set(REPLACE + REUSE), "Base manifest must contain all seven contracts.")
        require(all(re.fullmatch(r"C[A-Z2-7]{55}", v) for v in old.values()), "Invalid contract address.")
        admin = cli("keys", "address", self.deployer)
        require(admin == base["deployer"], "CLI identity does not match the base deployment admin.")
        # Check all existing admins and the two legacy grants before any writes.
        for name in REPLACE + REUSE:
            print(f"Checking existing {name}", flush=True)
            self.expect(old[name], "admin", admin)
            self.expect(old[name], "is_paused", False)
        self.expect(old["payment_distribution"], "get_token_contract", old["waste_token"])
        self.expect(old["waste_token"], "is_minter", True, "--minter", old["payment_distribution"])
        self.expect(old["reputation"], "is_operator", True, "--address", old["waste_transaction"])

        hashes = {n: hashlib.sha256((self.wasm_dir / f"{n}.wasm").read_bytes()).hexdigest() for n in REPLACE}
        if self.output.exists():
            journal = load(self.output)
            require(journal.get("previous_contracts") == old, "Journal belongs to a different base deployment.")
            require(journal.get("deployer") == admin and journal.get("network") == "testnet", "Journal identity/network mismatch.")
            require(journal.get("wasm_sha256") == hashes, "WASM changed since the rollout started; stop for review.")
            require(all(journal["contracts"][n] == old[n] for n in REUSE), "Journal replaced a preserved contract.")
        else:
            journal = {
                "network": "testnet", "rollout_state": "preparing",
                "deployment_date": datetime.now(timezone.utc).isoformat(),
                "source_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                "deployer": admin, "previous_contracts": old, "contracts": dict(old),
                "wasm_sha256": hashes, "salts": {n: secrets.token_hex(32) for n in REPLACE},
            }
            # Capture changes to deployed Rust source, but ignore rollout docs/tooling.
            if subprocess.run(["git", "diff", "--quiet", "HEAD", "--", "contracts", "Cargo.toml", "Cargo.lock"], cwd=ROOT).returncode:
                journal["source_commit"] += "+uncommitted"
            for name in REPLACE:
                journal["contracts"][name] = cli(
                    "contract", "id", "wasm", "--salt", journal["salts"][name],
                    "--source-account", self.deployer, *network_args(),
                ).strip('"')
            save(self.output, journal)

        current = journal["contracts"]
        require(len(set(current.values())) == 7, "The bundle must use seven distinct addresses.")
        for name in REPLACE:
            derived = cli("contract", "id", "wasm", "--salt", journal["salts"][name], "--source-account", self.deployer, *network_args()).strip('"')
            require(current[name] == derived and current[name] not in old.values(), "Journal salt/address mismatch.")
        for name in REPLACE:
            print(f"Preparing {name}: {current[name]}", flush=True)
            self.deploy(name, journal)
            extra = ("--token_contract", old["waste_token"]) if name == "payment_distribution" else ()
            self.ensure(current[name], "admin", admin, "initialize", write_args=("--admin", admin, *extra))
            self.expect(current[name], "is_paused", False)

        print("Wiring dependencies and adding roles; existing roles stay in place.", flush=True)
        for name in ("material_pricing", "reputation", "collector_registry", "collection_point"):
            self.ensure(current["waste_transaction"], f"get_{name}_contract", current[name], f"set_{name}_contract", write_args=("--contract_address", current[name]))
        self.expect(current["payment_distribution"], "get_token_contract", old["waste_token"])
        self.ensure(old["reputation"], "is_operator", True, "add_operator", read_args=("--address", current["waste_transaction"]), write_args=("--operator", current["waste_transaction"]))
        self.ensure(old["waste_token"], "is_minter", True, "add_minter", read_args=("--minter", current["payment_distribution"]), write_args=("--minter", current["payment_distribution"]))
        self.expect(old["waste_token"], "is_minter", True, "--minter", old["payment_distribution"])
        self.expect(old["reputation"], "is_operator", True, "--address", old["waste_transaction"])
        journal["rollout_state"] = "ready_for_app_cutover"
        save(self.output, journal)
        print(f"Candidate verified: {self.output}. App configuration has not changed.", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", type=Path, default=ROOT / "deployed_addresses_testnet.json")
    parser.add_argument("--out", type=Path, default=ROOT / "target/point-verification-testnet.json")
    parser.add_argument("--wasm-dir", type=Path, default=ROOT / "target/wasm32v1-none/release")
    parser.add_argument("--deployer", default="wastefi-deployer")
    parser.add_argument("--apply", action="store_true", help="Deploy three contracts, wire them and add roles on testnet.")
    args = parser.parse_args()
    require(args.base.resolve() != args.out.resolve(), "Output must not overwrite the active manifest.")
    require(args.out.resolve() != (ROOT / "deployed_addresses_testnet.json").resolve(), "Output must not overwrite the active manifest.")
    require(re.fullmatch(r"[A-Za-z0-9_-]{1,64}", args.deployer) and not re.fullmatch(r"S[A-Z2-7]{55}", args.deployer), "Use a CLI identity name, never a secret key.")
    if not args.apply:
        print("Plan only. Build collection_point, waste_transaction and payment_distribution first.")
        print("Apply adds three contracts and two roles; it preserves the token, balances and legacy roles.")
        print("It writes a candidate manifest, never the active deployment or application configuration.")
        return
    args.out.parent.mkdir(parents=True, exist_ok=True)
    with args.out.with_suffix(args.out.suffix + ".lock").open("a") as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            raise RuntimeError("Another rollout is using this journal.") from None
        Rollout(args.base, args.out, args.wasm_dir, args.deployer).run()


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

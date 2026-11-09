"""Exercise the derived payments rollout against a fake chain."""

import fcntl
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))
from test_point_rollout import Chain  # noqa: E402

SCRIPT = Path(__file__).resolve().parents[1] / "rollout-derived-payments-testnet.py"
SPEC = importlib.util.spec_from_file_location("derived_rollout", SCRIPT)
rollout = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(rollout)
shared = rollout.shared
CONTRACT = rollout.CONTRACT


class DerivedChain(Chain):
    """The real contract binds its transaction contract once; the base fake does not."""

    def cli(self, *args):
        if args[:2] == ("contract", "invoke") and args[args.index("--") + 1] == "set_transaction_contract":
            address = args[args.index("--id") + 1]
            if (address, "transaction") in self.bindings:
                raise RuntimeError("Transaction contract already set")
        return super().cli(*args)


class DerivedPaymentRolloutTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.directory = Path(self.tmp.name)
        self.base = json.loads((rollout.ROOT / "deployed_addresses_testnet.json").read_text())
        self.old = self.base["contracts"]
        self.base_path = self.directory / "base.json"
        self.base_path.write_text(json.dumps(self.base))
        self.output = self.directory / "candidate.json"
        data = CONTRACT.encode()
        (self.directory / f"{CONTRACT}.wasm").write_bytes(data)
        self.hashes = {CONTRACT: hashlib.sha256(data).hexdigest()}
        self.chain = DerivedChain(self.base, self.hashes)
        patcher = patch.object(shared, "cli", self.chain.cli)
        patcher.start()
        self.addCleanup(patcher.stop)
        self.subject = rollout.DerivedPaymentRollout(self.base_path, self.output, self.directory, "wastefi-deployer")

    def test_replaces_only_the_payment_contract_and_keeps_every_role(self):
        before = self.base_path.read_bytes()
        self.subject.run()
        candidate = shared.load(self.output)
        self.assertEqual(candidate["rollout_state"], "ready_for_app_cutover")
        new = candidate["contracts"][CONTRACT]
        self.assertNotEqual(new, self.old[CONTRACT])
        for name, address in self.old.items():
            if name != CONTRACT:
                self.assertEqual(candidate["contracts"][name], address)
        # The new contract is bound to the existing transaction contract and token.
        self.assertEqual(self.chain.bindings[(new, "transaction")], self.old["waste_transaction"])
        self.assertEqual(self.chain.bindings[(new, "token")], self.old["waste_token"])
        # It can mint, and the old payment contract still can, so pinned collections finish.
        self.assertIn(new, self.chain.minters)
        self.assertIn(self.old[CONTRACT], self.chain.minters)
        self.assertEqual(sum(method == "deploy" for _, method in self.chain.writes), 1)
        self.assertEqual(self.base_path.read_bytes(), before)

    def test_rerun_after_success_writes_nothing(self):
        self.subject.run()
        candidate = shared.load(self.output)
        writes = list(self.chain.writes)
        self.subject.run()
        self.assertEqual(self.chain.writes, writes)
        self.assertEqual(shared.load(self.output), candidate)

    def test_lost_deploy_and_initialization_responses_recover_the_same_address(self):
        self.chain.lose_deploy_response = True
        self.chain.lose_init_response = True
        self.subject.run()
        self.assertEqual(len(self.chain.deployed), 1)
        candidate = shared.load(self.output)
        self.assertIn(candidate["contracts"][CONTRACT], self.chain.deployed)
        self.assertEqual(candidate["rollout_state"], "ready_for_app_cutover")

    def test_a_different_existing_binding_stops_the_rollout(self):
        self.subject.run()
        new = shared.load(self.output)["contracts"][CONTRACT]
        # Someone bound the new contract elsewhere before a resume checked it.
        self.chain.bindings[(new, "transaction")] = self.old["collector_registry"]
        with self.assertRaisesRegex(RuntimeError, "get_transaction_contract"):
            self.subject.run()
        self.assertEqual(self.chain.bindings[(new, "transaction")], self.old["collector_registry"])

    def test_wrong_base_admin_stops_before_writes(self):
        self.chain.admins[self.old["waste_transaction"]] = "different admin"
        with self.assertRaisesRegex(RuntimeError, "admin"):
            self.subject.run()
        self.assertFalse(self.chain.writes)
        self.assertFalse(self.output.exists())

    def test_paused_payment_contract_stops_before_writes(self):
        self.chain.paused.add(self.old[CONTRACT])
        with self.assertRaisesRegex(RuntimeError, "is_paused"):
            self.subject.run()
        self.assertFalse(self.chain.writes)

    def test_missing_minter_role_on_the_old_contract_stops_before_writes(self):
        self.chain.minters.clear()
        with self.assertRaisesRegex(RuntimeError, "is_minter"):
            self.subject.run()
        self.assertFalse(self.chain.writes)

    def test_missing_wasm_stops_before_anything_is_saved(self):
        (self.directory / f"{CONTRACT}.wasm").unlink()
        with self.assertRaisesRegex(RuntimeError, "Build payment_distribution"):
            self.subject.run()
        self.assertFalse(self.chain.writes)
        self.assertFalse(self.output.exists())

    def test_changed_wasm_on_resume_stops(self):
        self.subject.run()
        (self.directory / f"{CONTRACT}.wasm").write_bytes(b"changed")
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "WASM changed"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_tampered_journal_cannot_replace_a_preserved_contract(self):
        self.subject.run()
        journal = shared.load(self.output)
        journal["contracts"]["waste_token"] = journal["contracts"][CONTRACT]
        shared.save(self.output, journal)
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "preserved"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_tampered_salt_cannot_target_a_different_contract(self):
        self.subject.run()
        journal = shared.load(self.output)
        journal["salts"][CONTRACT] = "00" * 32
        shared.save(self.output, journal)
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "salt/address"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_mismatched_deployed_wasm_stops_without_redeployment(self):
        self.subject.run()
        address = shared.load(self.output)["contracts"][CONTRACT]
        self.chain.deployed[address] = "unexpected hash"
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "Unexpected deployed WASM"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_the_active_manifest_cannot_be_the_output(self):
        active = str(rollout.ROOT / "deployed_addresses_testnet.json")
        with patch.object(sys, "argv", [str(SCRIPT), "--apply", "--out", active]):
            with self.assertRaisesRegex(RuntimeError, "Output must not overwrite"):
                rollout.main()

    def test_a_secret_key_is_never_accepted_as_the_deployer(self):
        secret_shaped = "S" + "A" * 55
        with patch.object(sys, "argv", [str(SCRIPT), "--apply", "--deployer", secret_shaped, "--out", str(self.output)]):
            with self.assertRaisesRegex(RuntimeError, "never a secret key"):
                rollout.main()

    def test_concurrent_process_cannot_use_the_same_journal(self):
        with self.output.with_suffix(".json.lock").open("a") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            with patch.object(sys, "argv", [str(SCRIPT), "--apply", "--out", str(self.output), "--wasm-dir", str(self.directory)]):
                with self.assertRaisesRegex(RuntimeError, "Another rollout"):
                    rollout.main()
        self.assertFalse(self.chain.writes)

    def test_plan_mode_changes_nothing(self):
        with patch.object(sys, "argv", [str(SCRIPT), "--out", str(self.output)]):
            rollout.main()
        self.assertFalse(self.chain.writes)
        self.assertFalse(self.output.exists())


if __name__ == "__main__":
    unittest.main()

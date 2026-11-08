"""Exercise interrupted deployments and the boundaries of the shared rollout."""

import base64
import fcntl
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import sys
import unittest
from unittest.mock import patch

SCRIPT = Path(__file__).resolve().parents[1] / "rollout-point-verification-testnet.py"
SPEC = importlib.util.spec_from_file_location("point_rollout", SCRIPT)
rollout = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(rollout)


class Chain:
    def __init__(self, base, hashes):
        self.admin = base["deployer"]
        self.old = base["contracts"]
        self.hashes = hashes
        self.deployed = {}
        self.admins = {address: self.admin for address in self.old.values()}
        self.paused = set()
        self.bindings = {(self.old["payment_distribution"], "token"): self.old["waste_token"]}
        self.minters = {self.old["payment_distribution"]}
        self.operators = {self.old["waste_transaction"]}
        self.writes = []
        self.lose_deploy_response = False
        self.lose_init_response = False

    def cli(self, *args):
        def arg(flag):
            return args[args.index(flag) + 1]

        if args[:2] == ("keys", "address"):
            return self.admin
        if args[:3] == ("contract", "id", "wasm"):
            return "C" + base64.b32encode(hashlib.sha256(arg("--salt").encode()).digest()).decode().rstrip("=") + "AAA"
        if args[:3] == ("contract", "info", "hash"):
            if arg("--id") not in self.deployed:
                raise RuntimeError("Unknown contract or failed RPC")
            return self.deployed[arg("--id")]
        if args[:2] == ("contract", "deploy"):
            address = self.cli("contract", "id", "wasm", "--salt", arg("--salt"))
            self.deployed[address] = self.hashes[Path(arg("--wasm")).stem]
            self.writes.append((address, "deploy"))
            if self.lose_deploy_response:
                raise RuntimeError("Submission response lost")
            return address
        if args[:2] != ("contract", "invoke"):
            raise AssertionError(args)
        address = arg("--id")
        method = args[args.index("--") + 1]
        if arg("--send") == "yes":
            self.writes.append((address, method))
            if method == "initialize":
                if address in self.admins:
                    raise RuntimeError("Already initialized")
                self.admins[address] = arg("--admin")
                if "--token_contract" in args:
                    self.bindings[(address, "token")] = arg("--token_contract")
                if self.lose_init_response:
                    raise RuntimeError("Initialization response lost")
            elif method.startswith("set_"):
                self.bindings[(address, method[4:-9])] = arg("--contract_address")
            elif method == "add_operator":
                self.operators.add(arg("--operator"))
            elif method == "add_minter":
                self.minters.add(arg("--minter"))
            else:
                raise AssertionError(f"Unexpected write: {method}")
            return "null"
        if method == "admin":
            if address not in self.admins:
                raise RuntimeError("Not initialized")
            value = self.admins[address]
        elif method == "is_paused":
            value = address in self.paused
        elif method == "is_minter":
            value = arg("--minter") in self.minters
        elif method == "is_operator":
            value = arg("--address") in self.operators
        elif method.startswith("get_") and method.endswith("_contract"):
            value = self.bindings.get((address, method[4:-9]))
        else:
            raise AssertionError(f"Unexpected read: {method}")
        return json.dumps(value)


class RolloutTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.directory = Path(self.tmp.name)
        self.base = json.loads((rollout.ROOT / "deployed_addresses_testnet.json").read_text())
        self.base_path = self.directory / "base.json"
        self.base_path.write_text(json.dumps(self.base))
        self.output = self.directory / "candidate.json"
        self.hashes = {}
        for name in rollout.REPLACE:
            data = name.encode()
            (self.directory / f"{name}.wasm").write_bytes(data)
            self.hashes[name] = hashlib.sha256(data).hexdigest()
        self.chain = Chain(self.base, self.hashes)
        patcher = patch.object(rollout, "cli", self.chain.cli)
        patcher.start()
        self.addCleanup(patcher.stop)
        self.subject = rollout.Rollout(self.base_path, self.output, self.directory, "wastefi-deployer")

    def test_resume_preserves_token_legacy_grants_and_active_manifest(self):
        before = self.base_path.read_bytes()
        self.subject.run()
        candidate = rollout.load(self.output)
        self.assertEqual(candidate["rollout_state"], "ready_for_app_cutover")
        for name in rollout.REUSE:
            self.assertEqual(candidate["contracts"][name], self.base["contracts"][name])
        self.assertEqual(len(self.chain.minters), 2)
        self.assertEqual(len(self.chain.operators), 2)
        self.assertIn(self.base["contracts"]["payment_distribution"], self.chain.minters)
        self.assertIn(self.base["contracts"]["waste_transaction"], self.chain.operators)
        writes = list(self.chain.writes)
        self.subject.run()
        self.assertEqual(self.chain.writes, writes)
        self.assertEqual(self.base_path.read_bytes(), before)
        self.assertEqual(rollout.load(self.output), candidate)

    def test_lost_deployment_and_initialization_responses_recover_same_addresses(self):
        self.chain.lose_deploy_response = True
        self.chain.lose_init_response = True
        self.subject.run()
        self.assertEqual(len(self.chain.deployed), 3)
        self.assertEqual(sum(method == "deploy" for _, method in self.chain.writes), 3)
        self.assertEqual(rollout.load(self.output)["rollout_state"], "ready_for_app_cutover")

    def test_wrong_base_admin_stops_before_writes(self):
        self.chain.admins[self.base["contracts"]["reputation"]] = "different admin"
        with self.assertRaisesRegex(RuntimeError, "admin"):
            self.subject.run()
        self.assertFalse(self.chain.writes)
        self.assertFalse(self.output.exists())

    def test_paused_base_stops_before_writes(self):
        self.chain.paused.add(self.base["contracts"]["waste_token"])
        with self.assertRaisesRegex(RuntimeError, "is_paused"):
            self.subject.run()
        self.assertFalse(self.chain.writes)

    def test_missing_legacy_role_stops_before_writes(self):
        self.chain.minters.clear()
        with self.assertRaisesRegex(RuntimeError, "is_minter"):
            self.subject.run()
        self.assertFalse(self.chain.writes)

    def test_failed_preflight_read_never_means_ready(self):
        with patch.object(self.subject, "call", side_effect=RuntimeError("RPC unavailable")):
            with self.assertRaises(RuntimeError):
                self.subject.run()
        self.assertFalse(self.chain.writes)

    def test_successful_void_mutation_accepts_empty_cli_stdout(self):
        with patch.object(rollout, "cli", return_value=""):
            self.assertIsNone(self.subject.call("address", "initialize", write=True))
            with self.assertRaises(json.JSONDecodeError):
                self.subject.call("address", "get_collection_point_contract")

    def test_empty_unset_binding_is_followed_by_set_and_explicit_readback(self):
        calls = []

        def response(address, method, *args, write=False):
            calls.append(method)
            if len(calls) == 1:
                raise json.JSONDecodeError("Empty optional CLI result", "", 0)
            return "expected" if not write else None

        with patch.object(self.subject, "call", side_effect=response):
            self.subject.ensure("address", "get_collection_point_contract", "expected", "set_collection_point_contract")
        self.assertEqual(calls, ["get_collection_point_contract", "set_collection_point_contract", "get_collection_point_contract"])

    def test_changed_wasm_on_resume_stops(self):
        self.subject.run()
        (self.directory / "collection_point.wasm").write_bytes(b"changed")
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "WASM changed"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_tampered_candidate_cannot_replace_existing_token(self):
        self.subject.run()
        journal = rollout.load(self.output)
        journal["contracts"]["waste_token"] = journal["contracts"]["collection_point"]
        rollout.save(self.output, journal)
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "preserved"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_mismatched_deployed_wasm_stops_without_redeployment(self):
        self.subject.run()
        journal = rollout.load(self.output)
        self.chain.deployed[journal["contracts"]["collection_point"]] = "unexpected hash"
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "Unexpected deployed WASM"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_tampered_salt_cannot_target_a_different_contract(self):
        self.subject.run()
        journal = rollout.load(self.output)
        journal["salts"]["collection_point"] = "00" * 32
        rollout.save(self.output, journal)
        writes = list(self.chain.writes)
        with self.assertRaisesRegex(RuntimeError, "salt/address"):
            self.subject.run()
        self.assertEqual(self.chain.writes, writes)

    def test_wrong_new_admin_cannot_be_reinitialized_or_receive_roles(self):
        self.subject.run()
        journal = rollout.load(self.output)
        transaction = journal["contracts"]["waste_transaction"]
        self.chain.admins[transaction] = "different admin"
        self.chain.operators.remove(transaction)
        with self.assertRaisesRegex(RuntimeError, "admin"):
            self.subject.run()
        self.assertEqual(self.chain.admins[transaction], "different admin")
        self.assertNotIn(transaction, self.chain.operators)
        self.assertIn(self.base["contracts"]["waste_transaction"], self.chain.operators)

    def test_concurrent_process_cannot_use_the_same_journal(self):
        with self.output.with_suffix(".json.lock").open("a") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            with patch.object(sys, "argv", [str(SCRIPT), "--apply", "--out", str(self.output)]):
                with self.assertRaisesRegex(RuntimeError, "Another rollout"):
                    rollout.main()
        self.assertFalse(self.chain.writes)


if __name__ == "__main__":
    unittest.main()

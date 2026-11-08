//! Point authorization across the real registry, transaction, reputation and payment contracts.
use collection_point::{CollectionPoint, CollectionPointClient};
use common::{MaterialType, PaymentStatus, TransactionStatus, VerificationStatus};
use payment_distribution::{PaymentDistribution, PaymentDistributionClient};
use reputation::{Reputation, ReputationClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal, String, TryFromVal,
};
use waste_token::{WasteToken, WasteTokenClient};
use waste_transaction::{WasteTransaction, WasteTransactionClient};

struct Fixture {
    env: Env,
    admin: Address,
    owner: Address,
    collector: Address,
    registry: Address,
    transactions: Address,
    reputation: Address,
    transaction_id: u64,
}

impl Fixture {
    fn new(configured: bool, status: Option<VerificationStatus>) -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let collector = Address::generate(&env);
        let registry = env.register(CollectionPoint, ());
        let transactions = env.register(WasteTransaction, ());
        let reputation = env.register(Reputation, ());
        let points = CollectionPointClient::new(&env, &registry);
        let tx = WasteTransactionClient::new(&env, &transactions);
        let rep = ReputationClient::new(&env, &reputation);
        points.initialize(&admin);
        tx.initialize(&admin);
        rep.initialize(&admin);
        tx.set_reputation_contract(&reputation);
        rep.add_operator(&transactions);
        if configured {
            tx.set_collection_point_contract(&registry);
        }
        if let Some(status) = status {
            let point_id = points.register_point(
                &owner,
                &String::from_str(&env, "Point A"),
                &String::from_str(&env, "Test location"),
                &soroban_sdk::vec![&env, MaterialType::Plastic],
            );
            match status {
                VerificationStatus::Verified => points.verify_point(&point_id),
                VerificationStatus::Pending => {
                    points.update_verification_status(&point_id, &VerificationStatus::Pending);
                }
                VerificationStatus::Rejected => {
                    points.update_verification_status(&point_id, &VerificationStatus::Pending);
                    points.update_verification_status(&point_id, &VerificationStatus::Rejected);
                }
                VerificationStatus::Unverified => {}
            }
        }
        let transaction_id = tx.record_collection(
            &collector,
            &owner,
            &MaterialType::Plastic,
            &2000,
            &8_000_000,
        );
        Self {
            env,
            admin,
            owner,
            collector,
            registry,
            transactions,
            reputation,
            transaction_id,
        }
    }

    fn tx(&self) -> WasteTransactionClient<'_> {
        WasteTransactionClient::new(&self.env, &self.transactions)
    }

    fn rep(&self) -> ReputationClient<'_> {
        ReputationClient::new(&self.env, &self.reputation)
    }

    /// Only this address may authorize this invocation, with no mocked subcalls.
    fn sign_as(&self, address: &Address, method: &str) {
        self.env.mock_auths(&[MockAuth {
            address,
            invoke: &MockAuthInvoke {
                contract: &self.transactions,
                fn_name: method,
                args: (self.transaction_id,).into_val(&self.env),
                sub_invokes: &[],
            },
        }]);
    }

    fn assert_unverified(&self) {
        let record = self.tx().get_transaction(&self.transaction_id);
        assert!(!record.verified);
        assert_eq!(record.status, TransactionStatus::Pending);
        assert_eq!(self.rep().get_statistics(&self.collector).1, 0);
    }
}

#[test]
fn verified_owner_signature_completes_the_record_and_reputation() {
    let f = Fixture::new(true, Some(VerificationStatus::Verified));
    f.sign_as(&f.owner, "verify_transaction_by_point");
    f.tx().verify_transaction_by_point(&f.transaction_id);
    // Capture invocation results before a getter replaces the last invocation.
    let auths = f.env.auths();
    let events = f.env.events().all();
    assert_eq!(auths.len(), 1);
    assert_eq!(auths[0].0, f.owner);
    assert!(events.iter().any(|(contract, topics, data)| {
        contract == f.transactions
            && topics == soroban_sdk::vec![&f.env, symbol_short!("tx_ver").into_val(&f.env)]
            && u64::try_from_val(&f.env, &data).unwrap() == f.transaction_id
    }));
    let (contract, topics, data) = events.last().unwrap();
    assert_eq!(contract, f.transactions);
    assert_eq!(
        topics,
        soroban_sdk::vec![
            &f.env,
            symbol_short!("verify_by").into_val(&f.env),
            f.transaction_id.into_val(&f.env)
        ]
    );
    assert_eq!(Address::try_from_val(&f.env, &data).unwrap(), f.owner);
    let record = f.tx().get_transaction(&f.transaction_id);
    assert!(record.verified);
    assert_eq!(record.status, TransactionStatus::Completed);
    assert_eq!(f.rep().get_statistics(&f.collector), (505, 1, 1, 0));
}

#[test]
fn another_verified_point_cannot_verify_the_record() {
    let f = Fixture::new(true, Some(VerificationStatus::Verified));
    let other = Address::generate(&f.env);
    let points = CollectionPointClient::new(&f.env, &f.registry);
    let id = points.register_point(
        &other,
        &String::from_str(&f.env, "Point B"),
        &String::from_str(&f.env, "Test location"),
        &soroban_sdk::vec![&f.env, MaterialType::Plastic],
    );
    points.verify_point(&id);
    f.sign_as(&other, "verify_transaction_by_point");
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    f.assert_unverified();
}

#[test]
fn the_owner_must_actually_authorize_verification() {
    let f = Fixture::new(true, Some(VerificationStatus::Verified));
    f.env.set_auths(&[]);
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    f.assert_unverified();
}

#[test]
fn admin_signature_cannot_replace_the_owner_on_the_point_path() {
    let f = Fixture::new(true, Some(VerificationStatus::Verified));
    f.sign_as(&f.admin, "verify_transaction_by_point");
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    f.assert_unverified();
}

#[test]
fn an_unregistered_owner_cannot_verify() {
    let f = Fixture::new(true, None);
    f.sign_as(&f.owner, "verify_transaction_by_point");
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    f.assert_unverified();
}

#[test]
fn unverified_pending_and_rejected_points_cannot_verify() {
    for status in [
        VerificationStatus::Unverified,
        VerificationStatus::Pending,
        VerificationStatus::Rejected,
    ] {
        let f = Fixture::new(true, Some(status));
        f.sign_as(&f.owner, "verify_transaction_by_point");
        assert!(f
            .tx()
            .try_verify_transaction_by_point(&f.transaction_id)
            .is_err());
        f.assert_unverified();
    }
}

#[test]
fn missing_registry_configuration_fails_closed() {
    let f = Fixture::new(false, Some(VerificationStatus::Verified));
    assert_eq!(f.tx().get_collection_point_contract(), None);
    f.sign_as(&f.owner, "verify_transaction_by_point");
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    f.assert_unverified();
}

#[test]
fn only_admin_can_configure_the_registry() {
    let f = Fixture::new(false, Some(VerificationStatus::Verified));
    f.env.mock_auths(&[MockAuth {
        address: &f.owner,
        invoke: &MockAuthInvoke {
            contract: &f.transactions,
            fn_name: "set_collection_point_contract",
            args: (f.registry.clone(),).into_val(&f.env),
            sub_invokes: &[],
        },
    }]);
    assert!(f
        .tx()
        .try_set_collection_point_contract(&f.registry)
        .is_err());
    assert_eq!(f.tx().get_collection_point_contract(), None);
    f.env.mock_all_auths();
    f.tx().set_collection_point_contract(&f.registry);
    assert_eq!(f.tx().get_collection_point_contract(), Some(f.registry));
}

#[test]
fn either_pause_blocks_point_verification() {
    for pause_registry in [false, true] {
        let f = Fixture::new(true, Some(VerificationStatus::Verified));
        if pause_registry {
            CollectionPointClient::new(&f.env, &f.registry).pause();
        } else {
            f.tx().pause();
        }
        f.sign_as(&f.owner, "verify_transaction_by_point");
        assert!(f
            .tx()
            .try_verify_transaction_by_point(&f.transaction_id)
            .is_err());
        f.assert_unverified();
    }
}

#[test]
fn a_point_cannot_revive_cancelled_or_disputed_records() {
    for status in [TransactionStatus::Cancelled, TransactionStatus::Disputed] {
        let f = Fixture::new(true, Some(VerificationStatus::Verified));
        f.tx().update_status(&f.transaction_id, &status);
        f.sign_as(&f.owner, "verify_transaction_by_point");
        assert!(f
            .tx()
            .try_verify_transaction_by_point(&f.transaction_id)
            .is_err());
        let record = f.tx().get_transaction(&f.transaction_id);
        assert!(!record.verified);
        assert_eq!(record.status, status);
        assert_eq!(f.rep().get_statistics(&f.collector).1, 0);
    }
}

#[test]
fn the_point_must_accept_the_records_material() {
    let mut f = Fixture::new(true, Some(VerificationStatus::Verified));
    f.transaction_id = f.tx().record_collection(
        &f.collector,
        &f.owner,
        &MaterialType::Metal,
        &2000,
        &8_000_000,
    );
    f.sign_as(&f.owner, "verify_transaction_by_point");
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    f.assert_unverified();
}

#[test]
fn reputation_failure_rolls_back_verification() {
    let f = Fixture::new(true, Some(VerificationStatus::Verified));
    f.rep().remove_operator(&f.transactions);
    f.sign_as(&f.owner, "verify_transaction_by_point");
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    f.assert_unverified();
}

#[test]
fn a_delivery_cannot_be_verified_twice() {
    let f = Fixture::new(true, Some(VerificationStatus::Verified));
    f.sign_as(&f.owner, "verify_transaction_by_point");
    f.tx().verify_transaction_by_point(&f.transaction_id);
    assert!(f
        .tx()
        .try_verify_transaction_by_point(&f.transaction_id)
        .is_err());
    assert_eq!(f.rep().get_statistics(&f.collector), (505, 1, 1, 0));
}

#[test]
fn admin_override_remains_explicit_and_requires_admin() {
    // Recovery for old records whose point key was never retained.
    let f = Fixture::new(false, None);
    f.sign_as(&f.owner, "verify_transaction");
    assert!(f.tx().try_verify_transaction(&f.transaction_id).is_err());
    f.assert_unverified();
    f.env.mock_all_auths();
    f.tx().pause();
    f.sign_as(&f.admin, "verify_transaction");
    f.tx().verify_transaction(&f.transaction_id);
    let (_, _, data) = f.env.events().all().last().unwrap();
    assert_eq!(Address::try_from_val(&f.env, &data).unwrap(), f.admin);
    assert!(f.tx().get_transaction(&f.transaction_id).verified);
    assert_eq!(f.rep().get_statistics(&f.collector), (505, 1, 1, 0));
}

#[test]
fn point_verified_delivery_can_still_be_paid_in_waste() {
    let f = Fixture::new(true, Some(VerificationStatus::Verified));
    f.sign_as(&f.owner, "verify_transaction_by_point");
    f.tx().verify_transaction_by_point(&f.transaction_id);
    let record = f.tx().get_transaction(&f.transaction_id);
    assert!(record.verified);
    assert_eq!(record.total_amount, 16_000_000);
    f.env.mock_all_auths();
    let token_id = f.env.register(WasteToken, ());
    let token = WasteTokenClient::new(&f.env, &token_id);
    token.initialize(
        &f.admin,
        &String::from_str(&f.env, "WasteFi Token"),
        &String::from_str(&f.env, "WASTE"),
        &7,
    );
    let payment_id = f.env.register(PaymentDistribution, ());
    let payments = PaymentDistributionClient::new(&f.env, &payment_id);
    payments.initialize(&f.admin, &token_id);
    token.add_minter(&payment_id);
    // Payout derivation is the next roadmap item; the current ABI still takes an amount.
    let payout = payments.process_payment(&f.transaction_id, &f.collector, &record.total_amount);
    payments.update_payment_status(&payout, &PaymentStatus::Completed);
    assert_eq!(token.balance(&f.collector), record.total_amount);
}

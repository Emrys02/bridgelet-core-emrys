#[cfg(test)]
mod test {
    use crate::{EphemeralAccountContract, EphemeralAccountContractClient, AccountStatus};
    use soroban_sdk::{testutils::Address as _, Address, Env, BytesN};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, EphemeralAccountContract);
        let client = EphemeralAccountContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let recovery = Address::generate(&env);
        let expiry_ledger = env.ledger().sequence() + 1000;

        client.initialize(&creator, &expiry_ledger, &recovery);
        let status = client.get_status();
        assert_eq!(status, AccountStatus::Active);
        assert_eq!(client.is_expired(), false);
    }

    #[test]
    fn test_record_payment() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, EphemeralAccountContract);
        let client = EphemeralAccountContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let recovery = Address::generate(&env);
        let asset = Address::generate(&env);
        let expiry_ledger = env.ledger().sequence() + 1000;

        client.initialize(&creator, &expiry_ledger, &recovery);
        client.record_payment(&100, &asset);

        let status = client.get_status();
        assert_eq!(status, AccountStatus::PaymentReceived);
    }

    #[test]
    fn test_multiple_payments() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, EphemeralAccountContract);
        let client = EphemeralAccountContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let recovery = Address::generate(&env);
        let asset1 = Address::generate(&env);
        let asset2 = Address::generate(&env);
        let expiry_ledger = env.ledger().sequence() + 1000;

        client.initialize(&creator, &expiry_ledger, &recovery);

        client.record_payment(&100, &asset1);
        let info = client.get_info();
        assert_eq!(info.payment_count, 1);

        client.record_payment(&50, &asset2);
        let info = client.get_info();
        assert_eq!(info.payment_count, 2);

        let status = client.get_status();
        assert_eq!(status, AccountStatus::PaymentReceived);
    }

    #[test]
    fn test_sweep_single_asset() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, EphemeralAccountContract);
        let client = EphemeralAccountContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let recovery = Address::generate(&env);
        let asset = Address::generate(&env);
        let destination = Address::generate(&env);
        let expiry_ledger = env.ledger().sequence() + 1000;

        client.initialize(&creator, &expiry_ledger, &recovery);
        client.record_payment(&100, &asset);

        let auth_sig = BytesN::from_array(&env, &[0u8; 64]);
        client.sweep(&destination, &auth_sig);

        let status = client.get_status();
        assert_eq!(status, AccountStatus::Swept);
    }
}
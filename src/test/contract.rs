use crate::contract::{DIDContract, DIDContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn create_contract() -> (Env, DIDContractClient<'static>) {
    let e: Env = Default::default();
    let did_contract = DIDContractClient::new(&e, &e.register_contract(None, DIDContract));

    (e, did_contract)
}

#[test]
fn test_initialize() {
    let (e, did_contract) = create_contract();

    let result =
        did_contract.initialize(&Address::random(&e), &String::from_slice(&e, "chaincerts"));

    let encoded_msi_len = 24;

    assert_eq!(
        result.len() as usize,
        "did:chaincerts:".len() + encoded_msi_len
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let (e, did_contract) = create_contract();
    let admin = Address::random(&e);
    let method = String::from_slice(&e, "chaincerts");
    did_contract.initialize(&admin, &method);
    did_contract.initialize(&admin, &method);
}

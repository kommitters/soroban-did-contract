use crate::did_document::{self, DIDDocument};
use crate::did_trait::DIDTrait;
use crate::did_uri;
use crate::error::ContractError;
use crate::service::Service;
use crate::storage;
use crate::verification_method::VerificationMethodEntry;
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, BytesN, Env, String, Vec,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

contractmeta!(
    key = "Description",
    val = "Smart contract for decentralized identifiers (DIDs)",
);

#[contract]
pub struct DIDContract;

#[contractimpl]
impl DIDTrait for DIDContract {
    fn initialize(
        e: Env,
        admin: Address,
        did_method: String,
        context: Vec<String>,
        verification_methods: Vec<VerificationMethodEntry>,
        services: Vec<Service>,
    ) -> DIDDocument {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        storage::write_admin(&e, &admin);

        let did_uri = did_uri::generate(&e, &did_method);
        did_document::set_initial_did_document(
            &e,
            &did_uri,
            &context,
            &verification_methods,
            &services,
        )
    }

    fn update_did(
        e: Env,
        context: Option<Vec<String>>,
        verification_methods: Option<Vec<VerificationMethodEntry>>,
        services: Option<Vec<Service>>,
    ) -> DIDDocument {
        validate_admin(&e);

        let mut did_document = storage::read_did_document(&e);

        did_document::update_did_document(
            &e,
            &context,
            &verification_methods,
            &services,
            &mut did_document,
        );

        did_document
    }

    fn get_did(e: Env) -> DIDDocument {
        storage::read_did_document(&e)
    }

    fn set_admin(
        e: Env,
        new_admin: Address,
        new_verification_methods: Option<Vec<VerificationMethodEntry>>,
    ) -> DIDDocument {
        validate_admin(&e);

        let mut did_document = storage::read_did_document(&e);

        did_document::update_did_document(
            &e,
            &Option::None,
            &new_verification_methods,
            &Option::None,
            &mut did_document,
        );

        storage::write_admin(&e, &new_admin);

        did_document
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        validate_admin(&e);

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    fn version(e: Env) -> String {
        String::from_str(&e, VERSION)
    }
}

fn validate_admin(e: &Env) {
    let contract_admin = storage::read_admin(e);
    contract_admin.require_auth();
}

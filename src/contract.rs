use crate::did_document;
use crate::did_document::DIDDocument;
use crate::did_trait::DIDTrait;
use crate::did_uri;
use crate::error::ContractError;
use crate::service::Service;
use crate::storage;
use crate::verification_method::VerificationMethod;
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, Env, String, Vec,
};

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;

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
        verification_methods: Vec<VerificationMethod>,
        services: Vec<Service>,
    ) -> DIDDocument {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }

        validate_emtpy_values(&e, &context, &verification_methods);

        storage::write_admin(&e, &admin);

        e.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);

        let did_uri = did_uri::generate(&e, &did_method);
        did_document::set_initial_did_document(&e, did_uri, context, verification_methods, services)
    }

    fn update_did(
        e: Env,
        admin: Address,
        context: Option<Vec<String>>,
        verification_methods: Option<Vec<VerificationMethod>>,
        services: Option<Vec<Service>>,
    ) -> DIDDocument {
        let contract_admin = storage::read_admin(&e);
        if contract_admin != admin {
            panic_with_error!(e, ContractError::NotAuthorized)
        }
        admin.require_auth();

        let mut did_document = storage::read_did_document(&e);

        did_document::update_did_document(
            &e,
            context,
            verification_methods,
            services,
            &mut did_document,
        );

        did_document
    }

    fn get_did(e: Env) -> DIDDocument {
        storage::read_did_document(&e)
    }
}


fn validate_emtpy_values(e: &Env, context: &Vec<String>, verification_methods: &Vec<VerificationMethod>) {
    if context.is_empty() {
        panic_with_error!(e, ContractError::EmptyContext);
    }

    if verification_methods.is_empty() {
        panic_with_error!(e, ContractError::EmptyVerificationMethods);
    }
}

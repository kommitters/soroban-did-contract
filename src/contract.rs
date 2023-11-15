use crate::did_trait::DIDTrait;
use crate::did_uri;
use crate::error::ContractError;
use crate::service::{format_services, Service};
use crate::storage;
use crate::verification_method::{format_verification_method, VerificationMethod};
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
    ) -> String {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        storage::write_admin(&e, &admin);

        let did_uri = did_uri::generate(&e, &did_method);
        storage::write_did_uri(&e, &did_uri);

        set_context(&e, &context);
        set_verification_methods(&e, &verification_methods, &did_uri);
        set_services(&e, &services, &did_uri);

        e.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);

        did_uri
    }

    fn update_did(
        e: Env,
        admin: Address,
        context: Option<Vec<String>>,
        verification_methods: Option<Vec<VerificationMethod>>,
        services: Option<Vec<Service>>,
    ) {
        let contract_admin = storage::read_admin(&e);
        if contract_admin != admin {
            panic_with_error!(e, ContractError::NotAuthorized)
        }
        admin.require_auth();

        let did_uri = storage::read_did_uri(&e);

        // Update only the fields that are not None
        if let Some(context) = context {
            set_context(&e, &context)
        }
        if let Some(verification_methods) = verification_methods {
            set_verification_methods(&e, &verification_methods, &did_uri)
        }
        if let Some(services) = services {
            set_services(&e, &services, &did_uri)
        }
    }

    fn get_did(e: Env) -> (Vec<String>, String, Vec<VerificationMethod>, Vec<Service>) {
        let context = storage::read_context(&e);
        let did_uri = storage::read_did_uri(&e);
        let verification_method = storage::read_verification_methods(&e);
        let services = storage::read_services(&e);

        (context, did_uri, verification_method, services)
    }
}

fn set_context(e: &Env, context: &Vec<String>) {
    if context.is_empty() {
        panic_with_error!(e, ContractError::EmptyContext);
    }
    storage::write_context(e, context);
}

fn set_verification_methods(
    e: &Env,
    verification_methods: &Vec<VerificationMethod>,
    did_uri: &String,
) {
    if verification_methods.is_empty() {
        panic_with_error!(e, ContractError::EmptyVerificationMethods);
    }

    let new_verification_methods = format_verification_method(e, verification_methods, did_uri);
    storage::write_verification_methods(e, &new_verification_methods);
}

fn set_services(e: &Env, services: &Vec<Service>, did_uri: &String) {
    let new_services: Vec<Service> = format_services(e, services, did_uri);
    storage::write_services(e, &new_services);
}

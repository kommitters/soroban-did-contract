use crate::did_trait::DIDTrait;
use crate::did_uri;
use crate::error::ContractError;
use crate::storage;
use crate::service::Service;
use crate::verification_method::{VerificationMethod, Controller};
use soroban_sdk::{ contract, contractimpl, panic_with_error, Address, Env, String, Vec };

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;

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

        // verify empty vectors
        verify_context(&e, &context);
        verify_verification_methods(&e, &verification_methods);

        update_did_attributes(&e, Some(context), Some(verification_methods), Some(services), &did_uri);

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
        if contract_admin == admin {
            admin.require_auth();
        }
        else {
            panic_with_error!(e, ContractError::NotAuthorized)
        }
        let did_uri = storage::read_did_uri(&e);

        update_did_attributes(&e, context, verification_methods, services, &did_uri);
    }

    fn get_did(e: Env) -> (Vec<String>, String, Vec<VerificationMethod>, Vec<Service>) {
        let context = storage::read_context(&e);
        let did_uri = storage::read_did_uri(&e);
        let verification_method = storage::read_verification_methods(&e);
        let services = storage::read_services(&e);

        (context, did_uri, verification_method, services)
    }
}

fn update_did_attributes(
        e: &Env,
        context: Option<Vec<String>>,
        verification_methods: Option<Vec<VerificationMethod>>,
        services: Option<Vec<Service>>,
        did_uri: &String,
    ) {
        if let Some(context_vec) = context {
            verify_context(e, &context_vec);
            storage::write_context(&e, &context_vec);
        }

        if let Some(verification_methods_vec) = verification_methods {
            verify_verification_methods(e, &verification_methods_vec);
            let mut updated_verification_methods: Vec<VerificationMethod> = Vec::new(&e);

            for mut verification_method in verification_methods_vec.iter() {
                verification_method.id = did_uri::concat_field_id(&e, &did_uri, &verification_method.id);
                if verification_method.controller.len() == 0 {
                    // verification_method.controller = Controller::Some(did_uri.clone()); // Cuando se especifica a Controller con el typo de verification_method.controller
                    verification_method.controller = did_uri.clone();

                }
                updated_verification_methods.push_back(verification_method);
            }
            storage::write_verification_methods(&e, &updated_verification_methods);
        }

        if let Some(services_vec) = services {
            let mut updated_services: Vec<Service> = Vec::new(&e);

            for mut service in services_vec.iter() {
                service.id = did_uri::concat_field_id(&e, &did_uri, &service.id);
                updated_services.push_back(service);
            }
            storage::write_services(&e, &updated_services);
        }
}

fn verify_context(e: &Env, context: &Vec<String>) {
    if context.is_empty() {
        panic_with_error!(e, ContractError::EmptyContext);
    }
}

fn verify_verification_methods(e: &Env, verification_methods: &Vec<VerificationMethod>) {
    if verification_methods.is_empty() {
        panic_with_error!(e, ContractError::EmptyVerificationMethod);
    }
}

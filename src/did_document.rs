use crate::error::ContractError;
use crate::service::{self, Service};
use crate::storage;
use crate::verification_method::{
    add_verification_methods, VerificationMethod, VerificationMethodEntry,
};
use soroban_sdk::{contracttype, panic_with_error, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DIDDocument {
    pub id: String,
    pub context: Vec<String>,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub assertion_method: Vec<String>,
    pub key_agreement: Vec<String>,
    pub capability_invocation: Vec<String>,
    pub capability_delegation: Vec<String>,
    pub service: Vec<Service>,
}

pub fn set_initial_did_document(
    e: &Env,
    did_uri: &String,
    context: &Vec<String>,
    verification_methods: &Vec<VerificationMethodEntry>,
    services: &Vec<Service>,
) -> DIDDocument {
    validate_context(e, context);
    validate_verification_methods(e, verification_methods);

    let mut did_document = DIDDocument {
        id: did_uri.clone(),
        context: context.clone(),
        verification_method: Vec::new(e),
        authentication: Vec::new(e),
        assertion_method: Vec::new(e),
        key_agreement: Vec::new(e),
        capability_invocation: Vec::new(e),
        capability_delegation: Vec::new(e),
        service: service::format_services(e, services, did_uri),
    };

    add_verification_methods(e, verification_methods, did_uri, &mut did_document);

    storage::write_did_document(e, &did_document);
    storage::extend_ttl_to_did_document(e);

    did_document
}

pub fn update_did_document(
    e: &Env,
    context: &Option<Vec<String>>,
    verification_methods: &Option<Vec<VerificationMethodEntry>>,
    services: &Option<Vec<Service>>,
    did_document: &mut DIDDocument,
) {
    if let Some(context) = context {
        validate_context(e, context);

        did_document.context = context.clone();
    }

    if let Some(verification_methods) = verification_methods {
        validate_verification_methods(e, verification_methods);

        add_verification_methods(
            e,
            verification_methods,
            &did_document.id.clone(),
            did_document,
        );
    }

    if let Some(services) = services {
        did_document.service = service::format_services(e, services, &did_document.id);
    }

    storage::write_did_document(e, did_document);
}

fn validate_context(e: &Env, context: &Vec<String>) {
    if context.is_empty() {
        panic_with_error!(e, ContractError::EmptyContext);
    }
}

fn validate_verification_methods(e: &Env, verification_methods: &Vec<VerificationMethodEntry>) {
    if verification_methods.is_empty() {
        panic_with_error!(e, ContractError::EmptyVerificationMethods);
    }
}

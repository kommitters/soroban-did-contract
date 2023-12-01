use crate::service::{self, Service};
use crate::storage;
use crate::verification_method::{
    add_verification_methods, VerificationMethod, VerificationMethodOutput,
};
use soroban_sdk::{contracttype, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DIDDocument {
    pub did: String,
    pub context: Vec<String>,
    pub verification_method: Vec<VerificationMethodOutput>,
    pub authentication: Vec<String>,
    pub assertion_method: Vec<String>,
    pub key_agreement: Vec<String>,
    pub capability_invocation: Vec<String>,
    pub capability_delegation: Vec<String>,
    pub services: Vec<Service>,
}

pub fn set_initial_did_document(
    e: &Env,
    did_uri: String,
    context: Vec<String>,
    verification_methods: Vec<VerificationMethod>,
    services: Vec<Service>,
) -> DIDDocument {
    let mut did_document = DIDDocument {
        did: did_uri.clone(),
        context,
        verification_method: Vec::new(e),
        authentication: Vec::new(e),
        assertion_method: Vec::new(e),
        key_agreement: Vec::new(e),
        capability_invocation: Vec::new(e),
        capability_delegation: Vec::new(e),
        services: service::format_services(e, &services, &did_uri),
    };

    add_verification_methods(e, &verification_methods, &did_uri, &mut did_document);

    storage::write_did_document(e, &did_document);

    did_document
}

pub fn update_did_document(
    e: &Env,
    context: Option<Vec<String>>,
    verification_methods: Option<Vec<VerificationMethod>>,
    services: Option<Vec<Service>>,
    did_document: &mut DIDDocument,
) {
    if let Some(context) = context {
        did_document.context = context;
    }

    if let Some(verification_methods) = verification_methods {
        add_verification_methods(
            e,
            &verification_methods,
            &did_document.did.clone(),
            did_document,
        );
    }

    if let Some(services) = services {
        did_document.services = service::format_services(e, &services, &did_document.did);
    }

    storage::write_did_document(e, did_document);
}

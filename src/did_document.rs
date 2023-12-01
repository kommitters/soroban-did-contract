use crate::did_uri;
use crate::service::Service;
use crate::storage;
use crate::verification_method::{
    VerificationMethod, VerificationMethodOutput, VerificationRelationship,
};
use soroban_sdk::{contracttype, vec, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DidDocument {
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
) -> DidDocument {
    let mut did_document = DidDocument {
        did: did_uri.clone(),
        context: context,
        verification_method: Vec::new(e),
        authentication: vec![e, String::from_slice(e, "")],
        assertion_method: vec![e, String::from_slice(e, "")],
        key_agreement: vec![e, String::from_slice(e, "")],
        capability_invocation: vec![e, String::from_slice(e, "")],
        capability_delegation: vec![e, String::from_slice(e, "")],
        services: services,
    };

    add_verification_methods(e, &verification_methods, &did_uri, &mut did_document);

    storage::write_did_document(e, &did_document);

    did_document
}

fn add_verification_methods(
    e: &Env,
    verification_methods: &Vec<VerificationMethod>,
    did_uri: &String,
    did_document: &mut DidDocument,
) {
    did_document.verification_method = format_verification_method(&e, &verification_methods, &did_uri);

    for verification_method in verification_methods.iter() {
        for relationship in verification_method.verification_relationships {
            let value = vec![
                e,
                did_uri::concat_fragment(e, did_uri, &verification_method.id),
            ];

            format_relationship(did_document, relationship, value);
        }
    }
}

fn format_relationship(
    did_document: &mut DidDocument,
    relationship: VerificationRelationship,
    value: Vec<String>,
) {
    match relationship {
        VerificationRelationship::Authentication => {
            did_document.authentication = value;
        }
        VerificationRelationship::AssertionMethod => {
            did_document.assertion_method = value;
        }
        VerificationRelationship::KeyAgreement => {
            did_document.key_agreement = value;
        }
        VerificationRelationship::CapabilityInvocation => {
            did_document.capability_invocation = value;
        }
        VerificationRelationship::CapabilityDelegation => {
            did_document.capability_delegation = value;
        }
    }
}

pub fn format_verification_method(
    e: &Env,
    verification_methods: &Vec<VerificationMethod>,
    did_uri: &String,
) -> Vec<VerificationMethodOutput> {
    let mut new_verification_methods: Vec<VerificationMethodOutput> = Vec::new(e);

    for verification_method in verification_methods.iter() {
        new_verification_methods.push_back(VerificationMethodOutput {
            id: did_uri::concat_fragment(e, did_uri, &verification_method.id),
            type_: verification_method.type_,
            controller: verification_method.controller,
            public_key_multibase: verification_method.public_key_multibase,
        });
    }

    new_verification_methods
}

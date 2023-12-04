use crate::{did_document::DIDDocument, did_uri};
use soroban_sdk::{contracttype, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationRelationship {
    Authentication,
    AssertionMethod,
    KeyAgreement,
    CapabilityInvocation,
    CapabilityDelegation,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationMethodType {
    Ed25519VerificationKey2020,
    X25519KeyAgreementKey2020,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationMethodInDocument {
    pub id: String,
    pub type_: VerificationMethodType,
    pub controller: String,
    pub public_key_multibase: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationMethod {
    pub id: String,
    pub type_: VerificationMethodType,
    pub controller: String,
    pub public_key_multibase: String,
    pub verification_relationships: Vec<VerificationRelationship>,
}

pub fn add_verification_methods(
    e: &Env,
    verification_methods: &Vec<VerificationMethod>,
    did_uri: &String,
    did_document: &mut DIDDocument,
) {
    did_document.verification_method = format_verification_method(e, verification_methods, did_uri);
    clean_all_relationships(e, did_document);

    for verification_method in verification_methods.iter() {
        for relationship in verification_method.verification_relationships {
            let key = did_uri::concat_fragment(e, did_uri, &verification_method.id);
            add_relationship(did_document, relationship, key);
        }
    }
}

fn add_relationship(
    did_document: &mut DIDDocument,
    relationship: VerificationRelationship,
    value: String,
) {
    match relationship {
        VerificationRelationship::Authentication => {
            did_document.authentication.push_back(value);
        }
        VerificationRelationship::AssertionMethod => did_document.assertion_method.push_back(value),
        VerificationRelationship::KeyAgreement => did_document.key_agreement.push_back(value),
        VerificationRelationship::CapabilityInvocation => {
            did_document.capability_invocation.push_back(value)
        }
        VerificationRelationship::CapabilityDelegation => {
            did_document.capability_delegation.push_back(value)
        }
    }
}

fn clean_all_relationships(e: &Env, did_document: &mut DIDDocument) {
    did_document.authentication = Vec::new(e);
    did_document.assertion_method = Vec::new(e);
    did_document.key_agreement = Vec::new(e);
    did_document.capability_invocation = Vec::new(e);
    did_document.capability_delegation = Vec::new(e);
}

pub fn format_verification_method(
    e: &Env,
    verification_methods: &Vec<VerificationMethod>,
    did_uri: &String,
) -> Vec<VerificationMethodInDocument> {
    let mut new_verification_methods: Vec<VerificationMethodInDocument> = Vec::new(e);

    for verification_method in verification_methods.iter() {
        let id = did_uri::concat_fragment(e, did_uri, &verification_method.id);

        let controller = if verification_method.controller.len() == 0 {
            did_uri.clone()
        } else {
            verification_method.controller
        };

        new_verification_methods.push_back(VerificationMethodInDocument {
            id,
            type_: verification_method.type_,
            controller,
            public_key_multibase: verification_method.public_key_multibase,
        });
    }

    new_verification_methods
}

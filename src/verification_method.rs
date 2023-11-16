use soroban_sdk::{contracttype, Env, String, Vec};

use crate::did_uri;

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
pub struct VerificationMethod {
    pub id: String,
    pub type_: VerificationMethodType,
    pub controller: String,
    pub public_key_multibase: String,
    pub verification_relationships: Vec<VerificationRelationship>,
}

pub fn format_verification_method(
    e: &Env,
    verification_methods: &Vec<VerificationMethod>,
    did_uri: &String,
) -> Vec<VerificationMethod> {
    let mut new_verification_methods: Vec<VerificationMethod> = Vec::new(e);

    for mut verification_method in verification_methods.iter() {
        verification_method.id = did_uri::concat_fragment(e, did_uri, &verification_method.id);
        if verification_method.controller.len() == 0 {
            verification_method.controller = did_uri.clone();
        }
        new_verification_methods.push_back(verification_method);
    }

    new_verification_methods
}

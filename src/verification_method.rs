use soroban_sdk::{contracttype, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationRelationship {
    Authentication,
    Assertion,
    KeyAgreement,
    CapabilityInvocation,
    CapabilityDelegation,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationMethodType {
    Ed25519VerificationKey2020,
    X25519KeyAgreement2020,
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

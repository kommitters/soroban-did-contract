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
    X25519KeyAgreementKey2020,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Controller {
    None,
    Some(String),
}

impl Controller {
    pub fn len(&self) -> u32 {
        match self {
            Controller::None => 0,
            Controller::Some(s) => s.len(),
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationMethod {
    pub id: String,
    pub type_: VerificationMethodType,
    pub controller: String, // controller debería ser de tipo Controller
    pub public_key_multibase: String,
    pub verification_relationships: Vec<VerificationRelationship>,
}

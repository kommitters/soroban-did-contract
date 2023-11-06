use soroban_sdk::{contracttype, String};

// Service types are defined in https://www.w3.org/TR/did-spec-registries/#service-types.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceType {
    LinkedDomains,
    DIDComm,
    DIDCommMessaging,
    CredentialRegistry,
    OID4VCI,
    OID4VP,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Service {
    pub id: String,
    pub type_: ServiceType,
    pub service_endpoint: String,
}

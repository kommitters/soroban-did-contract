use soroban_sdk::{contracttype, Env, String, Vec};

use crate::did_uri;

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

pub fn format_services(e: &Env, services: &Vec<Service>, did_uri: &String) -> Vec<Service> {
    let mut new_services: Vec<Service> = Vec::new(e);

    for mut service in services.iter() {
        service.id = did_uri::concat_fragment(e, did_uri, &service.id);
        new_services.push_back(service);
    }

    new_services
}

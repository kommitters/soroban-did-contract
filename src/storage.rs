use crate::service::Service;
use crate::verification_method::VerificationMethod;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,               // Address
    DidUri,              // String
    Context,             // Vec<String>
    VerificationMethods, // Vec<VerificationMethod>
    Services,            // Vec<Service>
}

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn read_did_uri(e: &Env) -> String {
    let key = DataKey::DidUri;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_did_uri(e: &Env, did: &String) {
    let key = DataKey::DidUri;
    e.storage().instance().set(&key, did);
}

pub fn read_context(e: &Env) -> Vec<String> {
    let key = DataKey::Context;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_context(e: &Env, context: &Vec<String>) {
    let key = DataKey::Context;
    e.storage().instance().set(&key, context);
}

pub fn read_verification_methods(e: &Env) -> Vec<VerificationMethod> {
    let key = DataKey::VerificationMethods;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_verification_methods(e: &Env, verification_methods: &Vec<VerificationMethod>) {
    let key = DataKey::VerificationMethods;
    e.storage().instance().set(&key, verification_methods);
}

pub fn read_services(e: &Env) -> Vec<Service> {
    let key = DataKey::Services;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_services(e: &Env, services: &Vec<Service>) {
    let key = DataKey::Services;
    e.storage().instance().set(&key, services);
}

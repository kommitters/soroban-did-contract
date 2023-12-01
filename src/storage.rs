use crate::did_document::DIDDocument;
use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,       // Address
    DIDDocument, // DIDDocument
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

pub fn read_did_document(e: &Env) -> DIDDocument {
    let key = DataKey::DIDDocument;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_did_document(e: &Env, did_document: &DIDDocument) {
    let key = DataKey::DIDDocument;
    e.storage().instance().set(&key, did_document);
}

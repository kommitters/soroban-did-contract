use crate::did_document::DIDDocument;
use soroban_sdk::{contracttype, Address, Env};

// MAXIMUM ENTRY TTL:
// 31 days, 12 ledger close per minute.
// (12 * 60 * 24 * 31) - 1
const LEDGERS_TO_EXTEND: u32 = 535_679;

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
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_did_document(e: &Env, did_document: &DIDDocument) {
    let key = DataKey::DIDDocument;
    e.storage().persistent().set(&key, did_document);
}

pub fn extend_ttl_to_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGERS_TO_EXTEND, LEDGERS_TO_EXTEND);
}

pub fn extend_ttl_to_did_document(e: &Env) {
    let key = DataKey::DIDDocument;
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGERS_TO_EXTEND, LEDGERS_TO_EXTEND);
}

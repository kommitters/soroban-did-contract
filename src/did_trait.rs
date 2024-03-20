use crate::did_document::DIDDocument;
use crate::service::Service;
use crate::verification_method::VerificationMethodEntry;
use soroban_sdk::{Address, BytesN, Env, String, Vec};

pub trait DIDTrait {
    /// Initializes the DID Contract by generating the DID URI, setting the admin, and storing the DID Document.
    /// Returns the generated DID Document.
    fn initialize(
        e: Env,
        admin: Address,
        did_method: String,
        context: Vec<String>,
        verification_methods: Vec<VerificationMethodEntry>,
        services: Vec<Service>,
    ) -> DIDDocument;

    /// Updates the DID Document. This function can only be called by the admin.
    fn update_did(
        e: Env,
        admin: Address,
        context: Option<Vec<String>>,
        verification_methods: Option<Vec<VerificationMethodEntry>>,
        services: Option<Vec<Service>>,
    ) -> DIDDocument;

    /// Returns the DID Document.
    fn get_did(e: Env) -> DIDDocument;

    /// Upgrades WASM code.
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);

    /// Returns the version of the contract.
    fn version(e: Env) -> String;
}

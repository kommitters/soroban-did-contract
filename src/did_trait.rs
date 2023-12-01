use crate::did_document::DidDocument;
use crate::service::Service;
use crate::verification_method::VerificationMethod;
use soroban_sdk::{Address, Env, String, Vec};

pub trait DIDTrait {
    /// Initializes the DID Contract by generating the DID URI, setting the admin, and storing the DID Document.
    /// Returns the generated DID Document.
    fn initialize(
        e: Env,
        admin: Address,
        did_method: String,
        context: Vec<String>,
        verification_methods: Vec<VerificationMethod>,
        services: Vec<Service>,
    ) -> DidDocument;

    /// Updates the DID Document. This function can only be called by the admin.
    fn update_did(
        e: Env,
        admin: Address,
        context: Option<Vec<String>>,
        verification_methods: Option<Vec<VerificationMethod>>,
        services: Option<Vec<Service>>,
    ) -> DidDocument;

    /// Returns the DID Document.
    fn get_did(e: Env) -> DidDocument;
}

use crate::service::Service;
use crate::verification_method::VerificationMethod;
use soroban_sdk::{Address, Env, String, Vec};

pub trait DIDTrait {
    /// Initializes the DID Contract by generating the DID URI, setting the admin, and storing the DID attributes.
    /// Returns the generated DID URI.
    fn initialize(
        e: Env,
        admin: Address,
        did_method: String,
        context: Vec<String>,
        verification_methods: Vec<VerificationMethod>,
        services: Vec<Service>,
    ) -> String;

    // Updates the DID attributes. This function can only be called by the admin.
    fn update_did(
        e: Env,
        admin: Address,
        context: Option<Vec<String>>,
        verification_methods: Option<Vec<VerificationMethod>>,
        services: Option<Vec<Service>>,
    );

    // Returns the DID attributes: Context, DID URI, Verification Methods, and Services.
    fn get_did(e: Env) -> (Vec<String>, String, Vec<VerificationMethod>, Vec<Service>);
}

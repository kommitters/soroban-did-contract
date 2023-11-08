use crate::did_trait::DIDTrait;
use crate::did_uri;
use crate::error::ContractError;
use crate::storage;
// use crate::service::Service;
// use crate::verification_method::VerificationMethod;
use soroban_sdk::{ contract, contractimpl, panic_with_error, Address, Env, String };

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;

#[contract]
pub struct DIDContract;

#[contractimpl]
impl DIDTrait for DIDContract {
    fn initialize(
        e: Env,
        admin: Address,
        did_method: String
        // _context: Vec<String>,
        // _verification_methods: Vec<VerificationMethod>,
        // _services: Vec<Service>,
    ) -> String {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        storage::write_admin(&e, &admin);

        let did_uri = did_uri::generate(&e, &did_method);
        storage::write_did_uri(&e, &did_uri);

        e.storage().instance().bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);

        did_uri
    }
}

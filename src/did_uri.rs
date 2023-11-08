use crate::base32;
use soroban_sdk::{Bytes, Env, String};

pub fn generate(e: &Env, did_method: &String) -> String {
    let random_bytes: [u8; 15] = get_random_bytes(e);
    let mut method_specific_id = [0u8; 24];

    base32::encode(&mut method_specific_id, &random_bytes);

    concat_did_uri(e, did_method, &method_specific_id)
}

fn concat_did_uri(e: &Env, did_method: &String, method_specific_id: &[u8]) -> String {
    let prefix = String::from_slice(e, "did:");
    let prefix_len = prefix.len() as usize;

    let did_method_len = did_method.len() as usize;

    let msi_len = method_specific_id.len();

    let mut slice = [0u8; 100]; // should be big enough for the DID URI
    prefix.copy_into_slice(&mut slice[..prefix_len]);
    did_method.copy_into_slice(&mut slice[prefix_len..prefix_len + did_method_len]);
    slice[prefix_len + did_method_len] = b':';

    let did_uri_len = prefix_len + did_method_len + 1;

    let mut msi_bytes = Bytes::new(e);
    msi_bytes.extend_from_slice(method_specific_id);

    msi_bytes.copy_into_slice(&mut slice[did_uri_len..did_uri_len + msi_len]);
    let str_did_uri = core::str::from_utf8(slice[..did_uri_len + msi_len].as_ref()).unwrap();

    String::from_slice(e, str_did_uri)
}

fn get_random_bytes(e: &Env) -> [u8; 15] {
    let mut random_bytes = [0u8; 15];

    for byte in &mut random_bytes {
        *byte = e.prng().u64_in_range(0..256) as u8;
    }

    random_bytes
}

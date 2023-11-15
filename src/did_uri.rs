use crate::base32;
use soroban_sdk::{Bytes, Env, String};

pub fn generate(e: &Env, did_method: &String) -> String {
    let random_bytes: [u8; 15] = get_random_bytes(e);
    let mut method_specific_id = [0u8; 24];

    base32::encode(&mut method_specific_id, &random_bytes);

    concat_did_uri(e, did_method, &method_specific_id)
}

pub fn concat_fragment(
    e: &Env,
    did_uri: &String,
    id: &String,
) -> String {
    let hash_str = String::from_slice(e, "#");

    let did_uri_len = did_uri.len() as usize;
    let id_len = id.len() as usize;
    let combined_len = did_uri_len + id_len + 1;

    let mut slice: [u8; 100] = [0; 100]; // should be big enough for both strings combined
    did_uri.copy_into_slice(&mut slice[..did_uri_len]);
    hash_str.copy_into_slice(&mut slice[did_uri_len..did_uri_len + 1]);
    id.copy_into_slice(&mut slice[(did_uri_len + 1)..combined_len]);

    let id_string = String::from_slice(e, core::str::from_utf8(&slice[..combined_len]).unwrap());

    id_string
}

fn get_random_bytes(e: &Env) -> [u8; 15] {
    let mut random_bytes = [0u8; 15];

    for byte in &mut random_bytes {
        *byte = e.prng().u64_in_range(0..256) as u8;
    }

    random_bytes
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

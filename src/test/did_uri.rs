use crate::did_uri;
use soroban_sdk::{Env, String};

#[test]
fn test_concat_fragment() {
    let e: Env = Default::default();
    let did_uri = String::from_str(
        &e,
        "did:chaincerts:zEYJrMxWigf9boyeJMTRN4Ern8DJMoCXaLK77pzQmxVjf",
    );
    let id = String::from_str(&e, "chaincerts");
    let result = did_uri::concat_fragment(&e, &did_uri, &id);

    assert_eq!(
        result as String,
        String::from_str(
            &e,
            "did:chaincerts:zEYJrMxWigf9boyeJMTRN4Ern8DJMoCXaLK77pzQmxVjf#chaincerts"
        )
    )
}

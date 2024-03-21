use crate::test::setup::{build_did_document, DIDContractTest};
use crate::verification_method::{
    format_verification_method, VerificationMethodEntry, VerificationMethodType,
    VerificationRelationship,
};
use soroban_sdk::{testutils::Address as _, vec, Address, String, Vec};

// Length of the Method Specific ID (MSI) encoded in Base32
const ENCODED_MSI_LEN: usize = 24;

#[test]
fn test_initialize() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let did_document = contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let expected_did_document = build_did_document(
        &env,
        &did_document.id,
        &context,
        &verification_methods,
        &services,
    );

    assert_eq!(
        did_document.id.len() as usize,
        "did:chaincerts:".len() + ENCODED_MSI_LEN
    );

    assert_eq!(did_document, expected_did_document);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let DIDContractTest {
        env: _env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_initialize_with_empty_context() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context: _context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let empty_context = vec![&env];

    contract.initialize(
        &admin,
        &did_method,
        &empty_context,
        &verification_methods,
        &services,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_initialize_with_empty_verification_methods() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods: _verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let empty_verification_methods = vec![&env];

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &empty_verification_methods,
        &services,
    );
}

#[test]
fn test_get_did() {
    let DIDContractTest {
        env: _,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let did_document = contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    assert_eq!(contract.get_did(), did_document)
}

#[test]
fn test_update_context() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let new_context = vec![
        &env,
        String::from_str(&env, "https://www.w3.org/ns/did/v1"),
        String::from_str(&env, "https://w3id.org/security/suites/ed25519-2020/v1"),
        String::from_str(&env, "https://w3id.org/security/suites/x25519-2020/v1"),
        String::from_str(&env, "https://www.example.com/context/v1"),
    ];

    contract.update_did(
        &Option::Some(new_context.clone()),
        &Option::None,
        &Option::None,
    );

    assert_eq!(contract.get_did().context, new_context)
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_update_context_with_empty_vec_should_panic() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    contract.update_did(
        &Option::Some(Vec::new(&env)),
        &Option::None,
        &Option::None,
    );
}

#[test]
fn test_update_verification_methods() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let did_document = contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let new_verification_methods = vec![
        &env,
        VerificationMethodEntry {
            id: String::from_str(&env, "keys-1"),
            type_: VerificationMethodType::Ed25519VerificationKey2020,
            controller: String::from_str(&env, ""),
            public_key_multibase: String::from_str(
                &env,
                "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
            ),
            verification_relationships: vec![
                &env,
                VerificationRelationship::Authentication,
                VerificationRelationship::AssertionMethod,
            ],
        },
    ];

    contract.update_did(
        &Option::None,
        &Option::Some(new_verification_methods.clone()),
        &Option::None,
    );

    let formatted_verification_methods =
        format_verification_method(&env, &new_verification_methods, &did_document.id);

    assert_eq!(
        formatted_verification_methods,
        contract.get_did().verification_method
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_update_verification_methods_with_empty_vec_should_panic() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    contract.update_did(
        &Option::None,
        &Option::Some(Vec::new(&env)),
        &Option::None,
    );
}

#[test]
fn test_update_services() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let new_services = vec![&env];

    contract.update_did(
        &Option::None,
        &Option::None,
        &Option::Some(new_services.clone()),
    );

    let did_document = contract.get_did();

    assert_eq!(did_document.service, new_services)
}

#[test]
fn test_version() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let pkg_version: &str = env!("CARGO_PKG_VERSION");
    let expected_version = String::from_str(&env, pkg_version);
    assert_eq!(contract.version(), expected_version)
}

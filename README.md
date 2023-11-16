# Soroban DID Contract

[![Release Badge](https://img.shields.io/github/v/release/kommitters/soroban-did-contract?style=for-the-badge)](https://github.com/kommitters/soroban-did-contract/releases)
[![License Badge](https://img.shields.io/github/license/kommitters/soroban-did-contract?style=for-the-badge)](https://github.com/kommitters/soroban-did-contract/blob/main/LICENSE)
![Build Badge](https://img.shields.io/github/actions/workflow/status/kommitters/soroban-did-contract/ci.yml?branch=main&style=for-the-badge)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/soroban-did-contract?style=for-the-badge)](https://coveralls.io/github/kommitters/soroban-did-contract)
[![OSSF-Scorecard Score](https://img.shields.io/ossf-scorecard/github.com/kommitters/soroban-did-contract?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/soroban-did-contract)

Soroban Smart Contract for Decentralized Identifiers (DIDs) that adheres to the [W3C DIDs v1.0 specification][w3c-did-core].

The **DID Contract** allows to manage a decentralized digital identity within the Soroban & Stellar ecosystem.

## Smart Contract Function Parameters

This section describes the parameters and corresponding structures used in the smart contract functions.

### `admin`: `Address`

The address of the smart contract administrator. The admin is the only account that can update the DID attributes, and it is set during the initialization of the smart contract.

**Example**

```
GDDOIBULGMONQSKPTCGL7B67UYLBIO45UTJFHSQOO3M56BC6AGKGZPNO
```


### `did_method`: `String`

The DID method name is a string which will be used to create the DID URI. See [DID Syntax][did-syntax] for more details.

**Example**

```rust
"chaincerts"
```

### `context`: `Vec<String>`

The context is a list of URLs that usually define the version of the W3C DID specification used by the DID document, and the suites used as verification methods.

**Example**

```rust
[
  "https://www.w3.org/ns/did/v1",
  "https://w3id.org/security/suites/ed25519-2020/v1",
  "https://w3id.org/security/suites/x25519-2020/v1"
]
```

### `verification_methods`: `Vec<VerificationMethod>`

A DID document can express verification methods, such as cryptographic public keys, which can be used to authenticate or authorize interactions with the DID subject or associated parties. See [Verification Methods][verification-methods] for more details.

Verification Methods are represented as a vector of `VerificationMethod` structures.


**`VerificationMethod`**:
  - `id`: `String` — Identifier for the Verification Method. It is a concatenated string that contains the DID URI followed by a hash (#) and the specific id of the verification method. For the `initialize` and `update_did` functions, only the specific id of the verification method is expected, while for the `get_did` function, the entire concatenated id will be returned. Example:
    ```rust
    // Expected in `initialize` and `update_did`
    "keys-1"

    // Returned value in `get_did`.
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
    ```
  - `type_`: `VerificationMethodType` — Allowed values: `Ed25519VerificationKey2020`, `X25519KeyAgreementKey2020`.
  - `controller`: `String` — DID URI. An empty string is allowed, in which case the own DID URI will be set as the controller.
  - `public_key_multibase`: `String` — The public key encoded in Multibase format (Base58BTC encoded).
  - `verification_relationships`: `Vec<VerificationRelationship>` — Allowed values: `Authentication`, `AssertionMethod`, `KeyAgreement`, `CapabilityInvocation`, `CapabilityDelegation`.

**Example**

```rust
[
  {
    "id": "keys-1",
    "type_": "Ed25519VerificationKey2020",
    "controller": "",
    "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
    "verification_relationships": [
      "Authentication",
      "Assertion"
    ]
  }
]
```


### `services`: `Vec<Service>`

Services are used in DID documents to express ways of communicating with the DID subject or associated entities. A service can be any type of service the DID subject wants to advertise, including decentralized identity management services for further discovery, authentication, authorization, or interaction. See [DID Services][did-services] for more details.

The services are represented as a vector of `Service` structures.

**`Service`**:
  - `id`: `String` — Identifier for the Service. It is a concatenated string that contains the DID URI followed by a hash (#) and the specific id of the service. For the `initialize` and `update_did` functions, only the specific id of the service is expected, while for the `get_did` function, the entire concatenated id will be returned. Example:
    ```rust
    // Expected in `initialize` and `update_did`
    "chaincerts"

    // Returned value in `get_did`.
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#chaincerts"
    ```
  - `type_`: `ServiceType` — Allowed values: `LinkedDomains`, `DIDComm`, `DIDCommMessaging`, `CredentialRegistry`, `OID4VCI`, `OID4VP`.
  - `service_endpoint`: `String` — The service endpoint URL.


**Example**

```rust
[
  {
    "id": "chaincerts",
    "type_": "LinkedDomains",
    "service_endpoint": "https://chaincerts.co"
  }
]
```

## Smart Contract Functions

### Initialize

Initializes the DID Contract by generating the DID URI, setting the contract admin, and storing the DID attributes: context, verification methods, and services. The DID URI is generated by concatenating the DID method name and a pseudo-random value encoded in Base32.

Returns the DID URI as a string.

```rust
fn initialize(
    e: Env,
    admin: Address,
    did_method: String,
    context: Vec<String>,
    verification_methods: Vec<VerificationMethod>,
    services: Vec<Service>,
) -> String;
```

**Example**

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  initialize \
  --admin ADMIN_PUBLIC_KEY \
  --did_method '"chaincerts"' \
  --services '[{"id": "chaincerts", "type_": "LinkedDomains", "service_endpoint": "https://chaincerts.co"}]' \
  --verification_methods '[{"id": "keys-1", "type_": "Ed25519VerificationKey2020", "controller": "", "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ", "verification_relationships": ["Authentication", "Assertion"]}, {"id": "keys-2", "type_": "X25519KeyAgreementKey2020", "public_key_multibase": "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN", "controller": "",  "verification_relationships": ["KeyAgreement"]}]' \
  --context '["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/ed25519-2020/v1", "https://w3id.org/security/suites/x25519-2020/v1"]'


# Response: DID_URI
did:chaincerts:vyfrxab6umfxddlzl62jktu7
```

### Update DID

Updates the DID attributes in the storage for an initialized DID Contract. The admin account is the only party authorized to invoke this function.

The DID attributes that can be updated are: Context, Verification Methods, and Services.

You have the flexibility to update one or more attributes in the same invocation by providing the corresponding parameters. For attributes that are not intended to be updated, simply pass `None` in the respective parameter.

Verification Methods and Context must not be empty; otherwise, a contract error will be thrown.

```rust
fn update_did(
    e: Env,
    admin: Address,
    context: Option<Vec<String>>,
    verification_methods: Option<Vec<VerificationMethod>>,
    services: Option<Vec<Service>>,
);
```

**Example**

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  update_did \
  --admin ADMIN_PUBLIC_KEY \
  --services '[{"id": "chaincerts", "type_": "LinkedDomains", "service_endpoint": "https://chaincerts.co"}, {"id": "chaincerts_vault", "type_": "LinkedDomains", "service_endpoint": "https://vault.chaincerts.co"}]' \
  --verification_methods '[{"id": "keys-1", "controller": "", "type_": "Ed25519VerificationKey2020", "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ", "verification_relationships": ["Authentication", "Assertion"]}]' \
  --context '["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/ed25519-2020/v1"]'
```

### Get DID

Returns the DID attributes: Context, DID URI, Verification Methods, and Services, as a tuple.

**Example**

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  get_did


# Response: [CONTEXT, DID_URI, VERIFICATION_METHODS, SERVICES]
[["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"did:chaincerts:vyfrxab6umfxddlzl62jktu7",[{"controller":"did:chaincerts:vyfrxab6umfxddlzl62jktu7","id":"did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1","public_key_multibase":"z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ","type_":"Ed25519VerificationKey2020","verification_relationships":["Authentication","Assertion"]}],[{"id":"did:chaincerts:vyfrxab6umfxddlzl62jktu7#chaincerts","service_endpoint":"https://chaincerts.co","type_":"LinkedDomains"},{"id":"did:chaincerts:vyfrxab6umfxddlzl62jktu7#chaincerts_vault","service_endpoint":"https://vault.chaincerts.co","type_":"LinkedDomains"}]]
```

## Contract Errors

| Code | Error | Description |
| --- | --- | --- |
| 1 | `AlreadyInitialized` | Contract already initialized
| 2 | `NotAuthorized` | Invoker is not the contract admin
| 3 | `EmptyContext` | Context provided is an empty vector
| 4 | `EmptyVerificationMethods` | Verification Methods provided is an empty vector


## Development

### Pre-requirements

In order to develop and test the smart contract, you need to install Rust and Soroban CLI. The process is outlined in the Soroban setup documentation, which can be accessed at [Soroban setup][soroban-setup].

### Setup

1. Clone the repository:
    ```
    git clone git@github.com:kommitters/soroban-did-contract.git
    ```

2. Build the project and install dependencies:
    ```
    cd soroban-did-contract
    soroban contract build
    ```

3. Run tests:
    ```
    cargo test
    ```

### Deployment

1. Build the contract:
    ```
    soroban contract build
    ```

    This will generate a WASM file for the contract in the `target/wasm32-unknown-unknown/release/` directory.

2. Deploy using Soroban CLI:
    ```
    soroban contract deploy \
        --source SOURCE_ACCOUNT_SECRET_KEY \
        --rpc-url https://soroban-testnet.stellar.org:443 \
        --network-passphrase 'Test SDF Network ; September 2015' \
        --wasm target/wasm32-unknown-unknown/release/soroban-did-contract.wasm

    CONTRACT_ID
    ```

## Changelog

Features and bug fixes are listed in the [CHANGELOG][changelog] file.

## Code of conduct

We welcome everyone to contribute. Make sure you have read the [CODE_OF_CONDUCT][coc] before.

## Contributing

For information on how to contribute, please refer to our [CONTRIBUTING][contributing] guide.

## License

This library is licensed under an MIT license. See [LICENSE][license] for details.

## Acknowledgements

Made with 💙 by [kommitters Open Source](https://kommit.co)

[license]: https://github.com/kommitters/soroban-did-contract/blob/main/LICENSE
[coc]: https://github.com/kommitters/soroban-did-contract/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/soroban-did-contract/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/soroban-did-contract/blob/main/CONTRIBUTING.md
[w3c-did-core]: https://www.w3.org/TR/did-core/
[soroban-setup]: https://soroban.stellar.org/docs/getting-started/setup
[did-syntax]: https://www.w3.org/TR/did-core/#did-syntax
[verification-methods]: https://www.w3.org/TR/did-core/#verification-methods
[did-services]: https://www.w3.org/TR/did-core/#services

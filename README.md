# Soroban DID Smart Contract
Soroban Smart Contract for Decentralized Identifiers as standardized by [W3C DIDs v1.0 specification][w3c-did-core].

[![Release Badge](https://img.shields.io/github/v/release/kommitters/soroban-did-contract?style=for-the-badge)](https://github.com/kommitters/soroban-did-contract/releases)
[![License Badge](https://img.shields.io/github/license/kommitters/soroban-did-contract?style=for-the-badge)](https://github.com/kommitters/soroban-did-contract/blob/main/LICENSE)
![Build Badge](https://img.shields.io/github/actions/workflow/status/kommitters/soroban-did-contract/ci.yml?branch=main&style=for-the-badge)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/soroban-did-contract?style=for-the-badge)](https://coveralls.io/github/kommitters/soroban-did-contract)
[![OSSF-Scorecard Score](https://img.shields.io/ossf-scorecard/github.com/kommitters/soroban-did-contract?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/soroban-did-contract)

> [!IMPORTANT]
>  🤝
> In line with our commitment to contribute to the [Stellar community][stellar], we have developed this DID smart contract that serves as an interface. This contract can be utilized by anyone seeking to innovate with a solution that follows the W3C specification.

## Features
The DID contract enables you to manage a Decentralized Identifier within the Soroban & Stellar ecosystem. With this smart contract, you will be able to:

- Create a DID.
- Update the DID attributes.
- Retrieve the DID document.

## Types

### VerificationMethodType
Defines the type of verification method associated with a DID subject.

> [!TIP]
>  Allowed values: `Ed25519VerificationKey2020`, `X25519KeyAgreementKey2020`

### VerificationRelationship
Expresses the relationship between the DID subject and a verification method.

> [!TIP]
>  Allowed values: `Authentication`, `AssertionMethod`, `KeyAgreement`, `CapabilityInvocation`, `CapabilityDelegation`.

### VerificationMethodEntry
Represents a cryptographic key or method used for verification purposes.

This type is used as parameter to set the verification methods in the [Initialize](#initialize) and [Update DID](#update-did) functions.

#### Attributes

| Name                     | Type                           | Values                                                      |
| ------------------------ | ------------------------------ | ----------------------------------------------------------- |
| `id`                     | `String`                       | Arbitrary identifier (e.g., `keys-1`).                       |
| `type_`                  | `VerificationMethodType`       | See [VerificationMethodType](#verificationmethodtype).      |
| `controller`             | `String`                       | If the DID URI is not provided, the DID URI from the contract is set as the controller.  |
| `public_key_multibase`   | `String`                       | Public key encoded in Multibase format (Base58BTC).         |
| `verification_relationships` | `Vec<VerificationRelationship>` | List of [VerificationRelationships](#verificationrelationship). |

#### Example
```bash
{
  "id": "keys-1",
  "type_": "Ed25519VerificationKey2020",
  "controller": "",
  "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
  "verification_relationships": ["Authentication", "AssertionMethod"]
}
```

### VerificationMethod
Represents a cryptographic key or method used for verification purposes.

This type is used to represent a verification method in the [DIDDocument](#diddocument) type.

#### Attributes

| Name                     | Type                           | Values                                                      |
| ------------------------ | ------------------------------ | ----------------------------------------------------------- |
| `id`                     | `String`                       | DID URI appended with an arbitrary identifier. |
| `type_`                  | `VerificationMethodType`       | See [VerificationMethodType](#verificationmethodtype).      |
| `controller`             | `String`                       | DID URI of the key controller.
| `public_key_multibase`   | `String`                       | Public key encoded in Multibase format (Base58BTC).         |

#### Example
```bash
{
  "id": "did:chaincerts:565s4nk6hch3jxlqjtn3e4il#keys-1",
  "type_": "Ed25519VerificationKey2020",
  "controller": "did:chaincerts:565s4nk6hch3jxlqjtn3e4il",
  "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
}
```

### ServiceType
Defines the service type associated with a DID.

> [!TIP]
>  Allowed values: `LinkedDomains`, `DIDComm`, `DIDCommMessaging`, `CredentialRegistry`, `OID4VCI`, `OID4VP`.

### Service
Extends the functionality of DIDs by providing detailed information about specific services associated with a DID.

#### Attributes

| Name                     | Type                           | Values                           |
| ------------------------ | ------------------------------ | ---------------------------------|
| `id`                     | `String`                       | Arbitrary identifier.            |
| `type_`                  | `ServiceType`                  | See [ServiceType](#servicetype). |
| `service_endpoint`       | `String`                       | The service endpoint URL.        |

#### Example
```bash
{
  "id": "chaincerts",
  "type_": "LinkedDomains",
  "service_endpoint": "https://chaincerts.co"
}
```

### DIDDocument

Represents a W3C DID document, which is a set of data that describes the DID subject.

#### Attributes

| Name                     | Type                           | Values                           |
| ------------------------ | ------------------------------ | ---------------------------------|
| `id`                     | `String`                       | DID URI generated on initialization.            |
| `context`                | `Vec<String>`                  | List of URLs defining W3C DID spec version and verification method suites. |
<<<<<<< Updated upstream
| `verification_method`    | `Vec<VerificationMethodInDocument>` | List of [VerificationMethodInDocument](#verificationmethodindocument). |
=======
| `verification_method`    | `Vec<VerificationMethod>`      | List of [VerificationMethods](#verificationmethod) |
>>>>>>> Stashed changes
| `authentication`         | `Vec<String>`                  | List of verification method ids for authentication. |
| `assertion_method`       | `Vec<String>`                  | List of verification method ids for assertion. |
| `key_agreement`          | `Vec<String>`                  | List of verification method ids for key agreement. |
| `capability_invocation`  | `Vec<String>`                  | List of verification method ids for capability invocation. |
| `capability_delegation`  | `Vec<String>`                  | List of verification method ids for capability delegation. |
| `service`                | `Vec<Service>`                 | List of [Services](#service).      |

### Example
```bash
{
  "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
  "context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1",
    "https://w3id.org/security/suites/x25519-2020/v1"
  ],
  "verification_method": [
    {
      "controller": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1",
      "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
      "type_": "Ed25519VerificationKey2020"
    },
    {
      "controller": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-2",
      "public_key_multibase": "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN",
      "type_": "X25519KeyAgreementKey2020"
    }
  ],
  "authentication": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "assertion_method": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "key_agreement": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-2"
  ],
  "capability_invocation": [],
  "capability_delegation": [],
  "service": [
    {
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#chaincerts",
      "service_endpoint": "https://chaincerts.co",
      "type_": "LinkedDomains"
    }
  ]
}
```

## Functions

### Initialize
Initializes the DID Contract by generating the DID URI, setting the contract admin, and storing the DID attributes: `Context`, `VerificationMethods`, and `Services`. The DID URI is generated by concatenating the DID method name with a random value encoded in Base32.

```rust
fn initialize(
    e: Env,
    admin: Address,
    did_method: String,
    context: Vec<String>,
    verification_methods: Vec<VerificationMethodEntry>,
    services: Vec<Service>
) -> DIDDocument;
```

#### Output
Returns a DID document.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  initialize \
  --admin ADMIN_PUBLIC_KEY \
  --did_method chaincerts \
  --services '[{"id": "chaincerts", "type_": "LinkedDomains", "service_endpoint": "https://chaincerts.co"}]' \
  --verification_methods '[{"id": "keys-1", "type_": "Ed25519VerificationKey2020", "controller": "", "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ", "verification_relationships": ["Authentication", "AssertionMethod"]}, {"id": "keys-2", "type_": "X25519KeyAgreementKey2020", "controller": "", "public_key_multibase": "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN", "verification_relationships": ["KeyAgreement"]}]' \
  --context '["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/ed25519-2020/v1", "https://w3id.org/security/suites/x25519-2020/v1"]'

# Output: DID DOCUMENT
{
  "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
  "context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1",
    "https://w3id.org/security/suites/x25519-2020/v1"
  ],
  "verification_method": [
    {
      "controller": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1",
      "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
      "type_": "Ed25519VerificationKey2020"
    },
    {
      "controller": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-2",
      "public_key_multibase": "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN",
      "type_": "X25519KeyAgreementKey2020"
    }
  ],
  "authentication": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "assertion_method": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "key_agreement": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-2"
  ],
  "capability_invocation": [],
  "capability_delegation": [],
  "service": [
    {
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#chaincerts",
      "service_endpoint": "https://chaincerts.co",
      "type_": "LinkedDomains"
    }
  ]
}
```

### Get DID
Provides the DID document.

```rust
fn get_did(e: Env) -> DIDDocument;
```

#### Output
Returns a DID document.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  get_did

# Output: DID DOCUMENT
{
  "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
  "context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1",
    "https://w3id.org/security/suites/x25519-2020/v1"
  ],
  "verification_method": [
    {
      "controller": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1",
      "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
      "type_": "Ed25519VerificationKey2020"
    },
    {
      "controller": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-2",
      "public_key_multibase": "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN",
      "type_": "X25519KeyAgreementKey2020"
    }
  ],
  "authentication": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "assertion_method": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "key_agreement": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-2"
  ],
  "capability_invocation": [],
  "capability_delegation": [],
  "service": [
    {
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#chaincerts",
      "service_endpoint": "https://chaincerts.co",
      "type_": "LinkedDomains"
    }
  ]
}
```

### Update DID

Updates the DID attributes in the storage for an initialized DID Contract. Only the admin account is authorized to invoke this function.

The updatable DID attributes include `Context`, `VerificationMethods`, and `Services`.

You have the flexibility to update one or more attributes in the same invocation by providing the corresponding parameters. For attributes that are not intended to be updated, simply pass `None` in the respective parameter.

Verification Methods and Context must not be empty; otherwise, a contract error will be thrown.


```rust
fn update_did(
    e: Env,
    admin: Address,
    context: Option<Vec<String>>,
    verification_methods: Option<Vec<VerificationMethodEntry>>,
    services: Option<Vec<Service>>
) -> DIDDocument;
```

#### Output
Returns a DID document.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  update_did \
  --admin ADMIN_PUBLIC_KEY \
  --services '[{"id": "ChaincertsVault", "type_": "LinkedDomains", "service_endpoint": "https://vault.chaincerts.co"}]' \
  --verification_methods '[{"id": "keys-1", "type_": "Ed25519VerificationKey2020", "controller": "", "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ", "verification_relationships": ["Authentication", "AssertionMethod"]}]' \
  --context '["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/ed25519-2020/v1"]'

# Output: DID DOCUMENT
{
  "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
  "context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1"
  ],
  "verification_method": [
    {
      "controller": "did:chaincerts:vyfrxab6umfxddlzl62jktu7",
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1",
      "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
      "type_": "Ed25519VerificationKey2020"
    }
  ],
  "authentication": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "assertion_method": [
    "did:chaincerts:vyfrxab6umfxddlzl62jktu7#keys-1"
  ],
  "key_agreement": [],
  "capability_invocation": [],
  "capability_delegation": [],
  "service": [
    {
      "id": "did:chaincerts:vyfrxab6umfxddlzl62jktu7#ChaincertsVault",
      "service_endpoint": "https://vault.chaincerts.co",
      "type_": "LinkedDomains"
    }
  ]
}
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
        --wasm target/wasm32-unknown-unknown/release/soroban_did_contract.wasm

    CONTRACT_ID
    ```

## Changelog

Features and bug fixes are listed in the [CHANGELOG][changelog] file.

## Code of conduct

We welcome everyone to contribute. Make sure you have read the [CODE_OF_CONDUCT][coc] before.

## Contributing

For information on how to contribute, please refer to our [CONTRIBUTING][contributing] guide.

## License

This software is licensed under the [MIT][license] © kommit.

<br/>

<hr/>

[<img src="https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/d60d775f-166b-4968-89b6-8be847993f8c" width="80px" alt="kommit"/>](https://kommit.co)

<sub>

[Website][kommit-website] •
[Github][kommit-github] •
[X][kommit-x] •
[LinkedIn][kommit-linkedin]

</sub>

[license]: https://github.com/kommitters/soroban-did-contract/blob/main/LICENSE
[coc]: https://github.com/kommitters/soroban-did-contract/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/soroban-did-contract/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/soroban-did-contract/blob/main/CONTRIBUTING.md
[w3c-did-core]: https://www.w3.org/TR/did-core/
[soroban-setup]: https://soroban.stellar.org/docs/getting-started/setup
[did-syntax]: https://www.w3.org/TR/did-core/#did-syntax
[verification-methods]: https://www.w3.org/TR/did-core/#verification-methods
[did-services]: https://www.w3.org/TR/did-core/#services
[kommit-website]: https://kommit.co
[kommit-github]: https://github.com/kommitters
[kommit-x]: https://twitter.com/kommitco
[kommit-linkedin]: https://www.linkedin.com/company/kommit-co
[stellar]: https://stellar.org

# Soroban DID Contract

[![Release Badge](https://img.shields.io/github/v/release/kommitters/soroban-did-contract?style=for-the-badge)](https://github.com/kommitters/soroban-did-contract/releases)
[![License Badge](https://img.shields.io/github/license/kommitters/soroban-did-contract?style=for-the-badge)](https://github.com/kommitters/soroban-did-contract/blob/main/LICENSE)
![Build Badge](https://img.shields.io/github/actions/workflow/status/kommitters/soroban-did-contract/ci.yml?branch=main&style=for-the-badge)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/soroban-did-contract?style=for-the-badge)](https://coveralls.io/github/kommitters/soroban-did-contract)
[![OSSF-Scorecard Score](https://img.shields.io/ossf-scorecard/github.com/kommitters/soroban-did-contract?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/soroban-did-contract)

Soroban Smart Contract for Decentralized Identifiers (DIDs) that adheres to the [W3C DIDs v1.0 specification][w3c-did-core]. The **DID Contract** allows to manage a decentralized digital identity within the Soroban & Stellar ecosystem.

## Smart contract function parameters

Below are the parameters and corresponding structures used in the smart contract functions.

- `admin`: `Address` - The address of the administrator for the smart contract.
    - Example:
        ```
        GDDOIBULGMONQSKPTCGL7B67UYLBIO45UTJFHSQOO3M56BC6AGKGZPNO
        ```
- `did_method`: `String` - The method used for decentralized identifier (DID) generation and management.
    - Example:
        ```rust
        "chaincerts"
        ```
- `context`: `Vec<String>` - A vector of strings representing the context of the smart contract.
    - Example:
        ```rust
        ["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/ed25519-2020/v1", "https://w3id.org/security/suites/x25519-2020/v1"]
        ```
- `verification_methods`: `Vec<VerificationMethod>` - A vector of `VerificationMethod` structures defining verification methods.
    - Verification method structure:
        - `id`: `String` - This will be concatenated with the DID URI to create an ID, for example: `did:chaincerts:zEYJrMxWigf9boyeJMTRN4Ern8DJMoCXaLK77pzQmxVjf#verification_method_id`.
        - `type_`: `VerificationMethodType` - Allowed values: `Ed25519VerificationKey2020`, `X25519KeyAgreementKey2020`.
        - `controller`: `String` - An empty string is allowed. In such cases, the value in the storage will be set as the DID URI.
        - `public_key_multibase`: `String`
        - `verification_relationships`: `Vec<VerificationRelationship>` - Allowed values: `Authentication`, `AssertionMethod`, `KeyAgreement`, `CapabilityInvocation`, `CapabilityDelegation`.
    - Example:
        ```rust
        [{"id": "keys-1", "type_": "Ed25519VerificationKey2020", "controller": "", "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ", "verification_relationships": ["Authentication", "Assertion"]}]
        ```
- `services`: `Vec<Service>` - A vector of `Service` structures representing services associated with the smart contract.
    - Service method structure:
        - `id`: `String` - This will be concatenated with the DID URI to create an ID, for example:  `did:chaincerts:zEYJrMxWigf9boyeJMTRN4Ern8DJMoCXaLK77pzQmxVjf#service_id`.
        - `type_`: `ServiceType` -  Allowed values: `LinkedDomain`, `DIDCom`, `DIDCommMessagin`, `CredentialRegistr`, `OID4VC`, `OID4VP`.
        - `service_endpoint`: `String`.
    - Example:
        ```rust
        [{"id": "chaincerts", "type_": "LinkedDomains", "service_endpoint": "https://chaincerts.co"}]
        ```

## Smart contract functions

### initialize

This function is responsible for initializing a deployed smart contract. It takes necessary parameters such as the administrator's address, context, verification methods, and services.

**Parameters**

- `admin`
- `did_method`
- `context`
- `verification_methods`
- `services`

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
  --services '[{"id": "chaincerts", "type_": "LinkedDomains", "service_endpoint": "https://chaincerts.co"}]' \
  --verification_methods '[{"id": "keys-1", "type_": "Ed25519VerificationKey2020", "controller": "", "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ", "verification_relationships": ["Authentication", "Assertion"]}, {"id": "keys-2", "type_": "X25519KeyAgreementKey2020", "public_key_multibase": "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN", "controller": "",  "verification_relationships": ["KeyAgreement"]}]' \
  --context '["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/ed25519-2020/v1", "https://w3id.org/security/suites/x25519-2020/v1"]'  \
  --did_method '"chaincerts"'
```

### Update DID

This function is responsible for updating the storage associated with the DID in an initialized smart contract. It requires necessary arguments like admin and optional ones such as context, verification methods, and services.

**Parameters**

- `admin`
- `context` **optional**
- `verification_methods` **optional**
- `services` **optional**

**Example**

```bash
soroban contract invoke \
  --id CBISS43ZCCK2SKBWSJEA3HNN3MV6JKKXBCRMB3NIU44QFAV4HVZ54TSP \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  update_did \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --services '[{"id": "chaincerts_updated", "type_": "LinkedDomains", "service_endpoint": "https://chaincerts.co"}]' \
  --verification_methods '[{"id": "keys-1_updated", "controller": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h", "type_": "Ed25519VerificationKey2020", "public_key_multibase": "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ", "verification_relationships": ["Authentication", "Assertion"]}, {"id": "keys-2_updated", "controller": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h", "type_": "X25519KeyAgreementKey2020", "public_key_multibase": "z6LSnL6WNE3cqZyWBqh9JTQ3DwWNNvXuNVD8oKZL8jdFyuWN", "verification_relationships": ["KeyAgreement"]}]' \
  --context '["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/ed25519-2020/v1", "https://w3id.org/security/suites/x25519-2020/v1"]'
```

### Get DID

This function is responsible for returning the smart contract storage related to the DID.

**Example**

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  get_did

[CONTEXT, DID_URI, VERIFICATION_METHODS, SERVICES]
```

## Development

### Pre-requirements
In order to utilize the contracts, it is necessary to have Rust and Soroban installed. The installation process is outlined in the Soroban setup documentation, which can be accessed at [Soroban setup][soroban-setup].

### Setup
1. Clone the repository:
```
git clone git@github.com:kommitters/soroban-did-contract.git
```

2. Build the project and install the dependencies:
```
cd soroban-did-contract
soroban contract build
```

3. Run tests:
```
cargo test -- --show-output
```

### Deployment
1. Build the smart contracts:
```
soroban contract build
```

2. Deploy using the Soroban CLI:
```
soroban contract deploy \
    --source SOURCE_ACCOUNT_SECRET_KEY \
    --rpc-url https://soroban-testnet.stellar.org:443 \
    --network-passphrase 'Test SDF Network ; September 2015' \
    --wasm target/wasm32-unknown-unknown/release/soroban-did-contract.wasm
SUCCESS
SUCCESS

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

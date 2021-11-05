import {RegistryTypes} from "@polkadot/types/types";

export const types: RegistryTypes = {
    Compose: {
      name: 'Vec<u8>',
      code_txt: 'Vec<u8>',
      exec_type: 'Vec<u8>',
      dest: 'AccountId',
      value: 'Balance',
      bytes: 'Vec<u8>',
      input_data: 'Vec<u8>',
    },
    GatewayVendor: {
      _enum: {
        Substrate: 0,
        Ethereum: 1
      }
    },
    GatewayType: {
      _enum: {
        Internal: 0,
        External: 1
      }
    },
    GatewayABIConfig: {
      block_number_type_size: 'u16',
      hash_size: 'u16',
      hasher: 'HasherAlgo',
      crypto: 'CryptoAlgo',
      address_length: 'u16',
      value_type_size: 'u16',
      decimals: 'u16',
      structs: 'Vec<StructDecl>',
    },
    GatewayGenesisConfig: {
      modules_encoded: 'Option<Vec<u8>>',
      signed_extension: 'Option<Vec<u8>>',
      runtime_version: 'RuntimeVersion',
      extrinsics_version: 'u8',
      genesis_hash: 'Vec<u8>',
    },
    StructDecl: {
      name: 'Type',
      fields: 'Vec<Parameter>',
      offsets: 'Vec<u16>',
    },
    HasherAlgo: {
      _enum: [
        "Blake2",
        "Keccak256"
      ]
    },
    CryptoAlgo: {
      _enum: [
        "Ed25519",
        "Sr25519",
        "Ecdsa"
      ]
    },
    CircuitOutboundMessage: {
      name: 'Bytes',
      module_name: 'Bytes',
      method_name: 'Bytes',
      sender: 'Option<Bytes>',
      target: 'Option<Bytes>',
      arguments: 'Vec<Bytes>',
      expected_output: 'Vec<GatewayExpectedOutput>',
      extra_payload: 'Option<ExtraMessagePayload>',
    },
    StorageItem: {
      key: 'Vec<Vec<u8>>',
      value: 'Vec<Option<Bytes>>',
    },
    EventsItem: {signatures: 'Vec<Bytes>'},
    ExtrinsicItem: {block_height: 'Option<u64>'},
    OutputItem: {output: 'Bytes'},
    GatewayExpectedOutput: {
      _enum: {
        Storage: "StorageItem",
        Events: "EventsItem",
        Extrinsic: "ExtrinsicItem",
        Output: "OutputItem",
      },
    },
    ProofTriePointer: {
      _enum: {
        State: 0,
        Transaction: 1,
        Receipts: 2,
      }
    },
    GatewayPointer: {
      id: 'ChainId',
      vendor: 'GatewayVendor',
      gateway_type: 'GatewayType',
    },
    ExtraMessagePayload: {
      signer: 'Bytes',
      module_name: 'Bytes',
      method_name: 'Bytes',
      call_bytes: 'Bytes',
      signature: 'Bytes',
      extra: 'Bytes',
      tx_signed: 'Bytes',
      custom_payload: 'Option<Bytes>',
    },
    ContractActionDesc: {
      action_id: 'Hash',
      target_id: 'Option<TargetId>',
      to: 'Option<AccountId>',
    },
    TargetId: 'ChainId'
}

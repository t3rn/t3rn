export default {
  types: {
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
      _enum: [ "Blake2", "Keccak256"]
    },
    CryptoAlgo: {
      _enum: ["Ed25519", "Sr25519", "Ecdsa",]
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
    GatewayExpectedOutput: {
      _enum: {
        Storage: {
          key: 'Vec<Vec<u8>>',
          value: 'Vec<Option<Bytes>>',
        },
        Events: {signatures: 'Vec<Bytes>'},
        Extrinsic: {
          block_height: 'Option<u64>',
        },
        Output: {output: 'Bytes'},
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
    // ContractMetadata: {
    //   metadata_version: 'Vec<u8>',
    //   name: 'Vec<u8>',
    //   version: 'Vec<u8>',
    //   authors: 'Vec<Vec<u8>>',
    //   description: 'Option<Vec<u8>>',
    //   documentation: 'Option<Vec<u8>>',
    //   repository: 'Option<Vec<u8>>',
    //   homepage: 'Option<Vec<u8>>',
    //   license: 'Option<Vec<u8>>',
    // }
  },
}

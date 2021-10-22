export default {
  rpc: {},
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
    StructDecl: {
      name: 'Type',
      fields: 'Vec<Parameter>',
      offsets: 'Vec<u16>',
    },
    HasherAlgo: {
      _enum: {
        Blake2: 0,
        Keccak256: 1
      }
    },
    CryptoAlgo: {
      _enum: {
        Ed25519: 0,
        Sr25519: 1,
        Ecdsa: 2,
      }
    },
    ContractMetadata: {
      metadata_version: 'Vec<u8>',
      name: 'Vec<u8>',
      version: 'Vec<u8>',
      authors: 'Vec<Vec<u8>>',
      description: 'Option<Vec<u8>>',
      documentation: 'Option<Vec<u8>>',
      repository: 'Option<Vec<u8>>',
      homepage: 'Option<Vec<u8>>',
      license: 'Option<Vec<u8>>',
    }
  },
}

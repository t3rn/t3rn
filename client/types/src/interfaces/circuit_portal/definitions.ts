export default {
  types: {
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
    HasherAlgo: {
      _enum: ['Blake2', 'Keccak256'],
    },
    CryptoAlgo: {
      _enum: ['Ed25519', 'Sr25519', 'Ecdsa'],
    },
    StructDecl: {
      name: 'Type',
      fields: 'Vec<Parameter>',
      offsets: 'Vec<u16>',
    },
  },
};

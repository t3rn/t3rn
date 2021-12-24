export default {
  types: {
    BlockNumber: 'u64',
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
      _enum: ['Substrate', 'Ethereum'],
    },
    GatewayType: {
      _enum: { ProgrammableInternal: 'u32', ProgrammableExternal: 'u32', TxOnly: 'u32' },
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
      modules_encoded: 'Option<Bytes>',
      // signed_extensions: 'Option<Bytes>',
      runtime_version: 'RuntimeVersion',
      extrinsics_version: 'u8',
      genesis_hash: 'Bytes',
    },
    StructDecl: {
      name: 'Type',
      fields: 'Vec<Parameter>',
      offsets: 'Vec<u16>',
    },
    HasherAlgo: {
      _enum: ['Blake2', 'Keccak256'],
    },
    CryptoAlgo: {
      _enum: ['Ed25519', 'Sr25519', 'Ecdsa'],
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
        Events: { signatures: 'Vec<Bytes>' },
        Extrinsic: {
          block_height: 'Option<u64>',
        },
        Output: { output: 'Bytes' },
      },
    },
    ProofTriePointer: {
      _enum: {
        State: 0,
        Transaction: 1,
        Receipts: 2,
      },
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
    TargetId: '[u8; 4]',
    SideEffect: {
      target: 'TargetId',
      prize: 'BalanceOf',
      ordered_at: 'BlockNumber',
      encoded_action: 'Bytes',
      encoded_args: 'Vec<Bytes>',
      signature: 'Bytes',
      enforce_executioner: 'Option<AccountId>',
    },
    ConfirmedSideEffect: {
      err: 'Option<Bytes>',
      output: 'Option<Bytes>',
      encoded_effect: 'Bytes',
      inclusion_proof: 'Option<Bytes>',
      executioner: 'AccountId',
      received_at: 'BlockNumber',
      cost: 'Option<BalanceOf>',
    },
    FullSideEffect: {
      input: 'SideEffect',
      confirmed: 'Option<ConfirmedSideEffect>',
    },
    GatewaySysProps: {
      ss58_format: 'u16',
      token_symbol: 'Bytes',
      token_decimals: 'u8',
    },
  },
};

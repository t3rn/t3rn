export default {
  types: {
    XtxId: 'Hash',
    result_status: 'Vec<u8>',
    Xtx: {
      estimated_worth: 'BalanceOf',
      current_worth: 'BalanceOf',
      requester: 'AccountId',
      escrow_account: 'AccountId',
      payload: 'Vec<u8>',
      current_step: 'u32',
      steps_no: 'u32',
      current_phase: 'u32',
      current_round: 'u32',
      schedule: 'XtxSchedule<AccountId,BlockNumber,Hash,BalanceOf>',
      phases_blockstamps: '(BlockNumber, BlockNumber)',
    },
    XtxSchedule: {
      result_status: 'Vec<u8>',
      phases_blockstamps: '(BlockNumber, BlockNumber)',
    },
    ProofType: {
      _enum: {
        FullValue: 0,
        MerklePath: 1,
      }
    },
    StepConfirmation: {
      step_index: 'u8',
      value: 'Bytes',
      proof: 'Proof',
      outbound_event: 'GatewayOutboundEvent',
    },
    GatewayOutboundEvent: {
      id: 'GatewayOutboundEventId',
      signature: 'Option<Vec<u8>>',
      namespace: 'Vec<u8>',
      name: 'Vec<u8>',
      data: 'Bytes',
      proof: 'Option<Proof>',
      args_abi: 'Vec<Type>',
      args_encoded: 'Vec<Bytes>',
      gateway_pointer: 'GatewayPointer',
    },
    GatewayOutboundEventId: 'u64',

  }
}

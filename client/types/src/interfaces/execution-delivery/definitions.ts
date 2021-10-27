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
    }
  }
}

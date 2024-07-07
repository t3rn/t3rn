import { z } from 'zod'

export const EvmGetBalanceSchema = z.object({
  endpoint: z
    .string({
      invalid_type_error: 'Endpoint must be a string',
      required_error: 'Endpoint is required',
    })
    .startsWith('ws://')
    .or(z.string().startsWith('wss://')),
  account: z
    .string({
      invalid_type_error: 'Account must be a string',
      required_error: 'Account is required',
    })
    .regex(/^0x[a-fA-F0-9]{40}$/, {
      message: 'Account must be a valid EVM address',
    }),
})

export const EvmTransferSchema = z.object({
  endpoint: z
    .string({
      invalid_type_error: 'Endpoint must be a string',
      required_error: 'Endpoint is required',
    })
    .startsWith('http://'),
  sender: z.string({
    invalid_type_error: 'Sender must be a string',
    required_error: 'Sender is required',
  }),
  receiver: z
    .string({
      invalid_type_error: 'Receiver must be a string',
      required_error: 'Receiver is required',
    })
    .regex(/^0x[a-fA-F0-9]{40}$/, {
      message: 'Receiver must be a valid EVM address',
    }),
  amount: z
    .number({
      invalid_type_error: 'Amount must be a number',
      required_error: 'Amount is required',
    })
    .positive(),
})

export const EvmDeploySchema = z.object({
  endpoint: z
    .string({
      invalid_type_error: 'Endpoint must be a string',
      required_error: 'Endpoint is required',
    })
    .startsWith('http://'),
  owner: z.string({
    invalid_type_error: 'Owner must be a string',
    required_error: 'Owner is required',
  }),
  contractAbi: z.string({
    invalid_type_error: 'Contract abi must be a string',
    required_error: 'Contract abi is required',
  }),
  contractBytecode: z.string({
    invalid_type_error: 'Contract bytecode must be a string',
    required_error: 'Contract bytecode is required',
  }),
})

export const EvmClaimAddressSchema = z.object({
  endpoint: z
    .string({
      invalid_type_error: 'Endpoint must be a string',
      required_error: 'Endpoint is required',
    })
    .startsWith('ws://')
    .or(z.string().startsWith('wss://')),
  signer: z.string({
    invalid_type_error: 'Signer must be a string',
    required_error: 'Signer is required',
  }),
})

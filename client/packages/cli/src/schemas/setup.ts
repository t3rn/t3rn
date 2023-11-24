import { z } from 'zod'

const TransferData = z.object({
  receiver: z.string({
    required_error: 'Receiver address is required',
    invalid_type_error: 'Receiver address must be a string',
  }),
  fee: z.optional(
    z.number({
      invalid_type_error: 'Fee must be a number',
    }),
  ),
})

const TokenInfo = z.record(
  z.object({
    symbol: z.string({
      required_error: 'Token symbol is required',
      invalid_type_error: 'Token symbol must be a string',
    }),
    decimals: z.number({
      required_error: 'Token decimals is required',
      invalid_type_error: 'Token decimals must be a number',
    }),
    id: z.optional(
      z.number({
        invalid_type_error: 'Token ID must be a number',
      }),
    ),
    address: z.optional(
      z.string({
        invalid_type_error: 'Token address must be a string',
      }),
    ),
  }),
)

const RegistrationData = z.object({
  owner: z.string({
    required_error: 'Owner address is required',
    invalid_type_error: 'Owner address must be a string',
  }),
  parachain: z.optional(
    z.object({
      relayChainId: z.string({
        required_error: 'Relay chain ID is required',
        invalid_type_error: 'Relay chain ID must be a string',
      }),
      id: z.number({
        required_error: 'Parachain ID is required',
        invalid_type_error: 'Parachain ID must be a number',
      }),
    }),
  ),
  verificationVendor: z.string({
    required_error: 'Verification vendor is required',
    invalid_type_error: 'Verification vendor must be a string',
  }),
  executionVendor: z.string({
    required_error: 'Execution vendor is required',
    invalid_type_error: 'Execution vendor must be a string',
  }),
  runtimeCodec: z.string({
    required_error: 'Runtime codec is required',
    invalid_type_error: 'Runtime codec must be a string',
  }),
  tokenInfo: TokenInfo,
  tokenLocation: z.string({
      required_error: 'Token location is required',
      invalid_type_error: 'Token location must be a string',
  }),
  allowedSideEffects: z.array(
    z.tuple([
      z.string({
        required_error: 'Allowed side effect is required',
        invalid_type_error: 'Allowed side effect must be a string',
      }),
      z.number({
        required_error: 'Allowed side effect is required',
        invalid_type_error: 'Allowed side effect must be a number',
      }),
    ]),
  ),
})

const GatewaySchema = z.object({
  name: z.string({
    required_error: 'Gateway name is required',
    invalid_type_error: 'Gateway name must be a string',
  }),
  id: z.string({
    required_error: 'Gateway ID is required',
    invalid_type_error: 'Gateway ID must be a string',
  }),
  rpc: z.optional(
    z.string({
      invalid_type_error: 'RPC endpoint must be a string',
    }),
  ),
  subscan: z.optional(
    z.string({
      invalid_type_error: 'Subscan URL must be a string',
    }),
  ),
  tokenId: z.number({
    required_error: 'Token ID is required',
    invalid_type_error: 'Token ID must be a number',
  }),
  transferData: TransferData,
  registrationData: RegistrationData,
})

export type Gateway = z.infer<typeof GatewaySchema>

const Circuit = z.object({
  decimals: z.number({
    required_error: 'Decimals are required',
    invalid_type_error: 'Decimals must be a number',
  }),
  valueTypeSize: z.number({
    required_error: 'Value type size is required',
    invalid_type_error: 'Value type size must be a number',
  }),
})

export const ConfigSchema = z.object({
  circuit: Circuit,
  gateways: z.array(GatewaySchema),
})

export type Config = z.infer<typeof ConfigSchema>

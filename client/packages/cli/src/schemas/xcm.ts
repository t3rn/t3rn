import { z } from 'zod'

export const XcmTransferSchema = z.object({
  signer: z.string({
    invalid_type_error: 'Signer must be a string',
    required_error: 'Signer is required',
  }),
  type: z.enum(['relay', 'system', 'para'], {
    invalid_type_error:
      'XCM transfer type must be a one of the following values: relay; system; para ',
    required_error: 'XCM transfer type is required',
  }),
  endpoint: z
    .string({
      invalid_type_error: 'Endpoint must be a string',
      required_error: 'Endpoint is required',
    })
    .startsWith('ws://')
    .or(z.string().startsWith('wss://')),
  dest: z
    .number({
      invalid_type_error: 'Destination chain ID must be a number',
      required_error: 'Destination chain ID is required',
    })
    .positive()
    .max(10000),
  recipient: z
    .string({
      required_error: 'Recipient is required',
      invalid_type_error: 'Recipient must be a string',
    })
    .regex(/0x[0-9a-f]{64}$/, {
      message:
        'Recipient value must be a valid substrate address in hex format',
    }),
  targetAmount: z
    .number({
      invalid_type_error: 'Target amount must be a number',
      required_error: 'Target amount is required',
    })
    .positive(),
  targetAsset: z
    .string({
      invalid_type_error: 'Target asset amount must be a string',
      required_error: 'Target asset is required',
    })
    .max(7)
    .regex(/^[a-zA-Z]+$/, {
      message: 'Token symbol must include only letters',
    }),
})

import { z } from 'zod'

export const XcmTransferSchema = z.object({
  signer: z.string({
    invalid_type_error: 'Signer must be a string',
    required_error: 'Signer is required',
  }),
  type: z
    .string({
      invalid_type_error: 'XCM transfer type must be a string',
      required_error: 'XCM transfer type is required',
    })
    .regex(/^(relay|system|para)$/),
  endpoint: z.string({
    invalid_type_error: 'Enpoint must be a string',
    required_error: 'Endpoint is required',
  }).startsWith('ws://'),
  dest: z
    .number({
      invalid_type_error: 'Destination chain ID must be a number',
      required_error: 'Destination chain ID is required',
    })
    .positive()
    .max(10000),
  recipient: z.string({
    required_error: 'Recipient is required',
    invalid_type_error: 'Recipient must be a string',
  })
  .regex(/0x[0-9a-f]{64}/),
  targetAmount: z.number({
    invalid_type_error: 'Target amount must be a number',
    required_error: 'Target amount is required',
  })
  .positive(),
  targetAsset: z.string({
    invalid_type_error: 'Target asset amount must be a string',
    required_error: 'Target asset is required',
  })
  .max(7)
  .regex(/^[a-zA-Z]+$/),
})

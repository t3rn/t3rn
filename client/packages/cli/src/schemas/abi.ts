import { z } from 'zod'

export const AddSfxAbiSchema = z.object({
  signer: z.string({
    invalid_type_error: 'Signer must be a string',
    required_error: 'Signer is required',
  }),
  endpoint: z.string({
    invalid_type_error: 'Enpoint must be a string',
    required_error: 'Endpoint is required',
  }),
  target: z
    .string({
      invalid_type_error: 'Destination chain ID must be a string',
      required_error: 'Destination chain ID is required',
    })
    .max(10),
  sfxId: z
    .string({
      invalid_type_error: 'SFX ID must be a string or hex number',
      required_error: 'SFX ID is required',
    })
    .max(10),
  sfxAbi: z.optional(
    z.string({
      invalid_type_error:
        'SFX ABI descriptor must be a string, but required if SFX ID is non-standard (not one of the built-in SFXs)',
      required_error:
        'SFX ABI descriptor is required - optional, but required if SFX ID is non-standard (not one of the built-in SFXs)',
    }),
  ),
  palletId: z.optional(
    z.number({
      invalid_type_error:
        'Pallet ID must be a number if provided -- Pallet ID on the destination chain that is responsible for generating events confirming SFX execution, e.g. 2 for Balances Pallet (must read from the runtime config")',
    }),
  ),
  purge: z.optional(
    z.any({
      invalid_type_error: 'Optional purge must be a boolean if provided',
    }),
  ),
})

import { z } from 'zod'

export const AssetRegistrationSchema = z.object({
  endpoint: z
    .string({
      invalid_type_error: 'Endpoint must be a string',
      required_error: 'Endpoint is required',
    })
    .startsWith('ws://')
    .or(z.string().startsWith('wss://')),
  dest: z.enum(['t0rn', 'local', 'para'], {
    invalid_type_error:
      'Destination type must be a one of the options: t0rn; local; para',
    required_error: 'Destination type is required',
  }),
  name: z.string({
    invalid_type_error: 'Name must be a string',
    required_error: 'Name is required',
  }),
  id: z
    .number({
      invalid_type_error: 'Token ID must be a number',
      required_error: 'Token ID is required',
    })
    .positive(),
  symbol: z
    .string({
      invalid_type_error: 'Token symbol must be a string',
      required_error: 'Token symbol is required',
    })
    .max(7)
    .regex(/^[a-zA-Z]+$/, {
      message: 'Token symbol must include only letters',
    }),
  decimals: z
    .number({
      invalid_type_error: 'Token decimals must be a number',
      required_error: 'Token decimals number is required',
    })
    .positive(),
})

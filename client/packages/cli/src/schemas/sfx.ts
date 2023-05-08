import { z } from "zod"

export const SideEffectSchema = z.object({
  target: z.string({
    required_error: "Target is required",
    invalid_type_error: "Target must be a string",
  }),
  type: z.string({
    required_error: "Type is required",
    invalid_type_error: "Type must be a string",
  }),
  from: z.string({
    required_error: "From is required",
    invalid_type_error: "From must be a string",
  }),
  to: z.string({
    required_error: "To is required",
    invalid_type_error: "To must be a string",
  }),
  amount: z
    .string({
      required_error: "Amount is required",
      invalid_type_error: "Amount must be a string",
    })
    .refine((value) => !isNaN(parseFloat(value)) && parseFloat(value) >= 0, {
      message: "Amount must be a non-negative number",
    }),
  insurance: z
    .string({
      required_error: "Insurance is required",
      invalid_type_error: "Insurance must be a string",
    })
    .refine((value) => !isNaN(parseFloat(value)) && parseFloat(value) >= 0, {
      message: "Insurance must be a non-negative number",
    }),
  reward: z
    .string({
      required_error: "Reward is required",
      invalid_type_error: "Reward must be a string",
    })
    .refine((value) => !isNaN(parseFloat(value)) && parseFloat(value) >= 0, {
      message: "Reward must be a non-negative number",
    }),
})

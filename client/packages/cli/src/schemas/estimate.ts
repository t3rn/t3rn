import { z } from "zod"
import { SideEffectActionSchema } from "./sfx.ts"

export const MaxRewardEstimateSchema = z.object({
  action: SideEffectActionSchema,
  baseAsset: z.string({
    invalid_type_error: "Base asset amount must be a string",
    required_error: "Base asset is required",
  }),
  target: z
    .string({
      required_error: "Target is required",
      invalid_type_error: "Target must be a string",
    })
    .min(4)
    .max(4),
  targetAmount: z.number({
    invalid_type_error: "Target amount must be a number",
    required_error: "Target amount is required",
  }),
  targetAsset: z.string({
    invalid_type_error: "Target asset amount must be a string",
    required_error: "Target asset is required",
  }),
  overSpend: z.number({
    invalid_type_error: "Over spend percent must be a number",
    required_error: "Over spend percent is required",
  }),
})

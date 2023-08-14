import { z } from "zod"

export const TransferEncodedArgsSchema = z.object({
  to: z.string({
    required_error: "From is required",
    invalid_type_error: "From must be a string",
  }),
  amount: z.number({
    required_error: "Amount is required",
    invalid_type_error: "Amount must be a number",
  }),
})

export type TransferEncodedArgs = z.infer<typeof TransferEncodedArgsSchema>

export const EncodedArgsSchema = z.custom(
  (value: any) => {
    // Try to parse the value with each schema. If any succeeds, return true. If all fail, return false.
    const schemas = [TransferEncodedArgsSchema]
    return schemas.some((schema) => {
      try {
        schema.parse(value)
        return true
      } catch (e) {
        return false
      }
    })
  },
  {
    message:
      "Encoded args must be a valid encoded args schema i.e (Transfer, Swap)",
  },
)

export type EncodedArgs = z.infer<typeof EncodedArgsSchema>

export const SideEffectActionSchema = z.enum([
  "data",
  "tran",
  "tass",
  "swap",
  "aliq",
  "rliq",
  "cevm",
  "wasm",
  "cgen",
])

export const SideEffectTargetSchema = z.enum(["eth", "roco", "sepl"])

export const SideEffectSchema = z.object({
  target: z
    .string({
      required_error: "Target is required",
      invalid_type_error: "Target must be a string",
    })
    .max(4),
  maxReward: z
    .string({
      required_error: "Max reward is required",
      invalid_type_error: "Max reward must be a string",
    })
    .refine((value) => !isNaN(parseFloat(value)) && parseFloat(value) >= 0, {
      message: "Max reward must be a non-negative number",
    }),
  insurance: z
    .string({
      required_error: "Insurance is required",
      invalid_type_error: "Insurance must be a string",
    })
    .refine((value) => !isNaN(parseFloat(value)) && parseFloat(value) >= 0, {
      message: "Insurance must be a non-negative number",
    }),
  action: SideEffectActionSchema,
  encodedArgs: z.array(EncodedArgsSchema),
  signature: z.string({
    invalid_type_error: "Signature must be a byte string",
  }),
  enforceExecutor: z
    .string({
      invalid_type_error: "Enforce executor must be a string",
    })
    .nullable(),
  rewardAssetId: z
    .string({
      invalid_type_error: "Reward asset id must be a string",
    })
    .nullable(),
})

export type SideEffect = z.infer<typeof SideEffectSchema>

export type SideEffectAction = SideEffect["action"]
export const SideEffectActions = {
  Data: "data",
  Transfer: "tran",
  TransferAsset: "tass",
  Swap: "swap",
  AddLiquidity: "aliq",
  RemoveLiquidity: "rliq",
  CallEvmContract: "cevm",
  CallWasmContract: "wasm",
  CallGenericAbi: "cgen",
} as const

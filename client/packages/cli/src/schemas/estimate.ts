import { z } from "zod"
import {
  SideEffectActionSchema,
  SideEffectTargetSchema,
} from "./sfx.ts"
import { ExtrinsicSchema } from "./extrinsic.ts"

export const MaxRewardEstimateSchema = z.object({
  action: SideEffectActionSchema,
  baseAsset: z.string({
    invalid_type_error: "Base asset amount must be a string",
    required_error: "Base asset is required",
  }),
  target: SideEffectTargetSchema,
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

export const EthSpeedModeSchema = z.enum(["rapid", "fast", "standard", "slow"])

export const EstimateEvmCallGasParamsSchema = z.object({
  fromAddress: z.string({
    invalid_type_error: "From address must be a string",
    required_error: "From address is required",
  }),
  toAddress: z.string({
    invalid_type_error: "To address must be a string",
    required_error: "To address is required",
  }),
  data: z.any({
    required_error: "Data (Bytes) is required",
  }),
  speedMode: EthSpeedModeSchema,
})

export const EstimateSubmittableExtrinsicSchema = z.object({
  sideEffect: ExtrinsicSchema,
  signer: z.string({
    invalid_type_error: "To address must be a string",
    required_error: "To address is required",
  })
})

export const EstimateSchema = z.object({
  action: SideEffectActionSchema,
  target: SideEffectTargetSchema,
  args: z.string().refine(
    (str) => {
      const isSpeedModeEnum = EthSpeedModeSchema.safeParse(str).success
      if (isSpeedModeEnum) return true
      const record = JSON.parse(str)
      const isEvmCallGasParams = EstimateEvmCallGasParamsSchema.safeParse(
        record
      ).success
      if (isEvmCallGasParams) return true
      return EstimateSubmittableExtrinsicSchema.safeParse(record).success
    },
    {
      message:
        "Args must be a valid speed mode, an EVM call estimation parameters or a side effect JSON string",
    },
  ),
})

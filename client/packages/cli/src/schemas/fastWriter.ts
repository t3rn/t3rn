import { z } from "zod"

export const FastWriterSchema = z.object({
  signer: z.string({
    invalid_type_error: "Signer must be a string",
    required_error: "Signer is required",
  }),
  endpoint: z.string({
    invalid_type_error: "Enpoint must be a string",
    required_error: "Endpoint is required",
  }),
  dest: z
    .string({
      invalid_type_error: "Destination chain ID must be a string",
      required_error: "Destination chain ID is required",
    })
    .max(10),
  source: z
    .string({
      invalid_type_error: "Source chain ID must be a string",
      required_error: "Source chain ID is required",
    })
    .max(10),
  targetAsset: z.number({
    invalid_type_error: "Asset ID must be a number (u32)",
    required_error: "Asset ID is required",
  }),
  targetAccount: z.string({
    required_error: "targetAccount is required",
    invalid_type_error: "targetAccount must be a string",
  }),
  targetAmount: z.number({
    required_error: "targetAmount is required",
    invalid_type_error: "targetAmount must be a number (u128)",
  }),
  rewardAsset: z.number({
    invalid_type_error: "rewardAsset ID must be a number (u32)",
    required_error: "rewardAsset ID is required",
  }),
  maxReward: z.number({
    invalid_type_error: "Target amount must be a number",
    required_error: "Target amount is required",
  }),
  insurance: z.number({
    invalid_type_error: "Insurance amount must be a string",
    required_error: "Insurance is required",
  }),
  speedMode: z.string({
    invalid_type_error:
      "speedMode must be a string (Instant, Fast, Rational, Finalized)",
    required_error:
      "speedMode is required (Instant, Fast, Rational, Finalized)",
  }),
  asUtilityBatch: z.optional(
    z.any({
      invalid_type_error:
        "asUtilityBatch is an option to send as a utility::batch call",
    }),
  ),
  asSequentialTx: z.optional(
    z.any({
      invalid_type_error:
        "asSequentialTx is an option to send as a sequence of transactions",
    }),
  ),
  asMultiSfx: z.optional(
    z.any({
      invalid_type_error:
        "asMultiSfx is an option to send as an XTX containing multiple of SFXs",
    }),
  ),
  repeat: z.optional(
    z.number({
      invalid_type_error: "repeat must be a number",
    }),
  ),
  repeatInterval: z.optional(
    z.number({
      invalid_type_error: "repeatInterval must be a number",
    }),
  ),
})

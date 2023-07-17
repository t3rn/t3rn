import { z } from "zod"
import { SideEffectSchema } from "./sfx.ts"

const SpeedModeEnum = z.enum(["Fast", "Rational", "Finalized"], {
  required_error: "Speed mode is required",
  invalid_type_error: "Speed mode must be one of Fast, Rational, Finalized",
})

export type SpeedMode = z.infer<typeof SpeedModeEnum>

export const SpeedModes = {
  Fast: "Fast",
  Rational: "Rational",
  Finalized: "Finalized",
} as const

export const ExtrinsicSchema = z.object({
  sideEffects: z.array(SideEffectSchema),
  speed_mode: SpeedModeEnum,
})

export type Extrinsic = z.infer<typeof ExtrinsicSchema>

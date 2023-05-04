import { z } from "zod"
import { SideEffectSchema } from "./sfx.ts"

export const ExtrinsicSchema = z.object({
  sideEffects: z.array(SideEffectSchema),
  sequential: z.boolean({
    required_error: "Sequential is required",
    invalid_type_error: "Sequential must be a boolean",
  }),
})

export type Extrinsic = z.infer<typeof ExtrinsicSchema>

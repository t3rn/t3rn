import { z } from "zod"
import { TransferSideEffectSchema } from "./transfer.ts"

export const ExtrinsicSchema = z.object({
  sideEffects: z.array(TransferSideEffectSchema),
  sequential: z.boolean({
    required_error: "Sequential is required",
    invalid_type_error: "Sequential must be a boolean",
  }),
})

export type Extrinsic = z.infer<typeof ExtrinsicSchema>

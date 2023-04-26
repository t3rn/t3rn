import { Args } from "@/types.ts"
import { handleSubmitExtrinsicCmd } from "./extrinsic.ts"

export const handleSubmitCmd = async (args: Args<"extrinsic">) => {
  if (args.extrinsic) {
    return handleSubmitExtrinsicCmd(args)
  }
}

import { Args } from "@/types.ts"
import { handleSubmitExtrinsicCmd } from "./extrinsic.ts"
import { handleSubmitHeadersCmd } from "./headers.ts"

export const handleSubmitCmd = async (args: Args<"extrinsic" | "headers">) => {
  if (args.extrinsic) {
    return handleSubmitExtrinsicCmd(args.extrinsic)
  }

  if (args.headers) {
    return handleSubmitHeadersCmd(args.headers)
  }
}

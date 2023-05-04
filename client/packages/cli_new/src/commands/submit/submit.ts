import { Args } from "@/types.ts"
import { handleSubmitSfxCmd } from "./sfx.ts"
import { handleSubmitHeadersCmd } from "./headers.ts"

export const handleSubmitCmd = async (args: Args<"sfx" | "headers">) => {
  if (args.sfx) {
    return handleSubmitSfxCmd(args.sfx)
  }

  if (args.headers) {
    return handleSubmitHeadersCmd(args.headers)
  }
}

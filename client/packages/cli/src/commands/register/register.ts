import { Args } from "@/types.ts"
import { log } from "@/utils/log.ts"
import { handleRegisterGateway } from "./gateway.ts"

export const handleRegisterCmd = async (args: Args<"gateway" | "export">) => {
  if (args.gateway) {
    return await handleRegisterGateway(args.gateway, Boolean(args.export))
  }

  log("ERROR", "No option provided!")
  process.exit(1)
}

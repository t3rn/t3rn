import { Args } from "@/types.ts"
import { getConfig } from "@/utils/config.ts"
import { handleRegisterGateway } from "./gateway.ts"

export const handleRegisterCmd = async (args: Args<"gateway">) => {
  const config = getConfig()
  if (!config) {
    process.exit(1)
  }

  if (args.gateway) {
    return await handleRegisterGateway(config, args.gateway)
  }
}

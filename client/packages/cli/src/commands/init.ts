import path from "path"
import { writeFileSync } from "fs"
import cleanStack from "clean-stack"
import { greet } from "@/utils/fns.ts"
import { log } from "@/utils/log.ts"
import transferTemplate from "@/templates/transfer.ts"
import { Args } from "@/types.ts"

export const handleInitCmd = async (args: Args<"config" | "transfer">) => {
  if (Object.keys(args).length === 0) {
    log("ERROR", "No arguments provided")
    process.exit(1)
  }

  console.log(greet())

  if (args.transfer) {
    initTransferFile(args.transfer)
  }
}

export const initTransferFile = (filePath: string | true) => {
  const template = JSON.stringify(transferTemplate, null, 2)

  try {
    const file = filePath === true ? path.join("./", "transfer.json") : filePath
    writeFileSync(file, template)
    log("SUCCESS", `${file} transfer template generated!`)
  } catch (e) {
    log(
      "ERROR",
      `Unable to generate transfer template. Reason: ${cleanStack(e.stack)}`,
    )
  }
}

import path from "path"
import { writeFileSync } from "fs"
import cleanStack from "clean-stack"
import { greet } from "@/utils/fns.ts"
import { log } from "@/utils/log.ts"
import setupTemplate from "@/templates/setup.ts"
import transferTemplate from "@/templates/transfer.ts"
import { CONFIG_FILE } from "@/consts.ts"
import { Args } from "@/types.ts"

export const handleInitCmd = async (args: Args<"config" | "transfer">) => {
  if (Object.keys(args).length === 0) {
    log("ERROR", "No arguments provided")
    process.exit(1)
  }

  console.log(greet())

  if (args.config) {
    initConfigFile(args.config)
  }

  if (args.transfer) {
    initTransferFile(args.transfer)
  }
}

export const initConfigFile = (filePath: string | true) => {
  const template = JSON.stringify(setupTemplate, null, 2)

  try {
    const file = filePath === true ? path.join("./", CONFIG_FILE) : filePath
    writeFileSync(file, template)
    log("SUCCESS", `@t3rn/cli config initialized at ${file}`)
  } catch (e) {
    log(
      "ERROR",
      `Unable to initialize t3rn CLI config. Reason: ${cleanStack(e.stack)}`,
    )
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

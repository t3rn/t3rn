import path from "path"
import { writeFileSync } from "fs"
import cleanStack from "clean-stack"
import { greet } from "@/utils/fns.ts"
import { log } from "@/utils/log.ts"
import transferTemplate from "@/templates/transfer.ts"
import xTransferTemplate from "@/templates/xtransfer.ts"
import { Args } from "@/types.ts"

export const handleInitCmd = async (
  args: Args<"config" | "transfer" | "xtransfer">,
) => {
  if (Object.keys(args).length === 0) {
    log("ERROR", "No arguments provided")
    process.exit(1)
  }

  console.log(greet())

  if (args.transfer) {
    initTransferFile(args.transfer)
  } else if (args.xtransfer) {
    initXcmTransferFile(args.xtransfer)
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

export const initXcmTransferFile = (filePath: string | true) => {
  const template = JSON.stringify(xTransferTemplate, null, 2)

  try {
    const file =
      filePath === true ? path.join("./", "xtransfer.json") : filePath
    writeFileSync(file, template)
    log("SUCCESS", `${file} XCM transfer template generated!`)
  } catch (e) {
    log(
      "ERROR",
      `Unable to generate transfer template. Reason: ${cleanStack(e.stack)}`,
    )
  }
}

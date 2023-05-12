import z from "zod"
import figlet from "figlet"
import { cryptoWaitReady } from "@t3rn/sdk"
import { log } from "./log.ts"
import cleanStack from "clean-stack"
import { CONFIG_FILE } from "@/consts.ts"

export const greet = () =>
  console.log(
    figlet.textSync("t3rn CLI", {
      font: "3D-ASCII",
    })
  )

export const validate = <T>(
  schema: z.ZodType<T>,
  data: Record<string, unknown>,
  { configFileName } = {
    configFileName: CONFIG_FILE,
  }
): T | undefined => {
  try {
    return schema.parse(data)
  } catch (error) {
    if (error instanceof z.ZodError) {
      log(
        "ERROR",
        "Invalid configuration provided in " +
        configFileName +
        ". Please review and make the necessary changes. \n" +
        "Validation failed with the following errors: \n" +
        error.errors
          .map((e) => `• ${e.message} (path: ${e.path.join(".")})`)
          .join("\n")
          .trim()
      )
    } else {
      log("ERROR", "An unexpected error occurred: " + cleanStack(error.message))
    }
  }
}

export const wrapCryptoWaitReady =
  (cb: (...args: any[]) => void) =>
    async (...args: any[]) => {
      try {
        const isReady = await cryptoWaitReady()

        if (isReady) {
          cb(...args)
        } else {
          throw new Error("Crypto is not ready")
        }
      } catch (err) {
        log("ERROR", cleanStack(err.message))
      }
    }

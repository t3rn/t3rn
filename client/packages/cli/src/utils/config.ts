import { existsSync, readFileSync } from "fs"
import path from "path"
import cleanStack from "clean-stack"
import { CONFIG_FILE } from "@/consts.ts"
import { Config, ConfigSchema } from "@/schemas/setup.ts"
import { log } from "./log.ts"
import { validate } from "./fns.ts"

export const checkForConfigFile = () => {
  try {
    const exists = existsSync(path.join("./", CONFIG_FILE))

    if (!exists) {
      throw new Error(
        `${CONFIG_FILE} file does not exist, run the 'init' command to generate it`,
      )
    }

    return true
  } catch (error) {
    log("ERROR", error.message)
  }

  return false
}

export const parseConfigFile = (
  path: string,
): Record<string, unknown> | undefined => {
  try {
    const config = readFileSync(path, "utf-8")
    return JSON.parse(config)
  } catch (error) {
    if (error instanceof SyntaxError) {
      log(
        "ERROR",
        `Unable to read ${CONFIG_FILE} file, please check what you have`,
      )
    } else {
      log("ERROR", cleanStack(error.message))
    }
  }
}

export const getConfig = (): Config | undefined => {
  const configExists = checkForConfigFile()
  if (!configExists) {
    return
  }

  const config = parseConfigFile(path.join("./", CONFIG_FILE))
  if (!config) {
    return
  }

  const validatedConfig = validate<Config>(ConfigSchema, config)
  return validatedConfig
}

import { existsSync, readFileSync } from 'fs'
import path from 'path'
import cleanStack from 'clean-stack'
import { Config, ConfigSchema } from '@/schemas/setup.ts'
import { log } from './log.ts'
import { validate } from './fns.ts'
import { config } from '@/config/config.ts'

export const checkForConfigFile = () => {
  try {
    const exists = existsSync(path.join('./', config().t3rnConfigFile))

    if (!exists) {
      throw new Error(
        `${
          config().t3rnConfigFile
        } file does not exist, run the 'init' command to generate it`,
      )
    }

    return true
  } catch (error) {
    log('ERROR', error.message)
  }

  return false
}

export const parseConfigFile = (
  path: string,
): Record<string, unknown> | undefined => {
  try {
    const t3rnConfig = readFileSync(path, 'utf-8')
    return JSON.parse(t3rnConfig)
  } catch (error) {
    if (error instanceof SyntaxError) {
      log(
        'ERROR',
        `Unable to read ${
          config().t3rnConfigFile
        } file, please check what you have`,
      )
    } else {
      log('ERROR', cleanStack(error.message))
    }
  }
}

export const getConfig = (): Config | undefined => {
  const configExists = checkForConfigFile()
  if (!configExists) {
    return
  }

  const t3rnConfig = parseConfigFile(path.join('./', config().t3rnConfigFile))
  if (!t3rnConfig) {
    return
  }

  const validatedConfig = validate<Config>(ConfigSchema, t3rnConfig)
  return validatedConfig
}

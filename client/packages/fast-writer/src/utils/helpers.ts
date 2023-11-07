import { logger } from './logger'

/**
 * Sleep for a given number of seconds.
 * @param {number}  seconds seconds to sleep for
 * @param {string=} reason reason for sleeping
 */
export function sleep(seconds: number, reason?: string): Promise<void> {
  logger.debug({ reason }, `💤 Sleeping for ${seconds} sec...`)
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve()
    }, seconds * 1000)
  })
}

export function asciiToBytes(str: string): Uint8Array {
  const bytesArray: number[] = []
  for (let i = 0; i < str.length; i++) {
    bytesArray.push(str.charCodeAt(i))
  }
  return new Uint8Array(bytesArray)
}

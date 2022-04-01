import { promisify } from 'util'
import { exec as _exec } from 'child_process'

export const exec = promisify(_exec)

export function formatEvents(
  events: { event: { section: string; method: string; data: any } }[]
): string[] {
  return events.map(
    ({ event: { data, method, section } }) =>
      `${section}.${method} ${data.toString()}`
  )
}

export function noop(): void {}

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

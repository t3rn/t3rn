import { tmpdir } from 'os'
import { join } from 'path'
import { writeFile } from 'fs/promises'
import { promisify } from 'util'
import { exec as _exec } from 'child_process'
import { JustificationNotification } from '@polkadot/types/interfaces'

export const exec = promisify(_exec)

export function formatEvents(
  events: { event: { section: string; method: string; data: any } }[]
): string[] {
  return events.map(
    ({ event: { data, method, section } }) =>
      `${section}.${method} ${data.toString()}`
  )
}

export async function grandpaDecode(justification: any) {
  const tmpFile = join(tmpdir(), justification.toString().slice(0, 10))

  await writeFile(tmpFile, justification.toString())

  return exec(
    './justification-decoder/target/release/justification-decoder ' + tmpFile
  ).then(cmd => JSON.parse(cmd.stdout))
}

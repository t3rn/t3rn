import { Args } from '@/types.ts'
import { log } from '@/utils/log.ts'
import { handleSubmitSfxCmd } from './sfx.ts'
import { handleSubmitHeadersCmd } from './headers.ts'

export const handleSubmitCmd = async (
  args: Args<'sfx' | 'headers' | 'export'>,
) => {
  if (args.sfx) {
    return handleSubmitSfxCmd(args.sfx, Boolean(args.export))
  }

  if (args.headers) {
    return handleSubmitHeadersCmd(args.headers, Boolean(args.export))
  }

  log('ERROR', 'No option provided!')
  process.exit(1)
}

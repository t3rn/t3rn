import { Args } from '@/types.ts'
import { log } from '@/utils/log.ts'
import { handleRegisterGateway } from './gateway.ts'

export const handleRegisterCmd = async (
  args: Args<'gateway' | 'export' | 'slot'>,
) => {
  if (!args.gateway) {
    log('ERROR', 'No gateway ID provided!')
    process.exit(1)
  }

  const slot = args.slot ? Number(args.slot) : undefined
  return await handleRegisterGateway(args.gateway, Boolean(args.export), slot)
}

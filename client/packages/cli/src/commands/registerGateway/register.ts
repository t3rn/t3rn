import { Args } from '@/types.ts'
import { log } from '@/utils/log.ts'
import { handleRegisterGateway } from './gateway.ts'

export const handleRegisterCmd = async (args: Args<'gateway' | 'export'>) => {
  if (!args) {
    log('ERROR', 'No gateway ID provided!')
    process.exit(1)
  }
  await handleRegisterGateway(args, false)
}

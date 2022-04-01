import Listener from './listener'
import Relayer from './relayer'

async function main() {
  const listener: Listener = new Listener()
  const relayer: Relayer = new Relayer()

  Listener.debug('🦅 remote endpoint', listener.kusamaEndpoint)
  Relayer.debug('⚡ circuit endpoint', relayer.circuitEndpoint)
  Listener.debug('⛩️  gateway id', listener.gatewayId.toString())
  Listener.debug('🏔️  range size', listener.rangeSize)

  Relayer.debug('initializing...')
  await relayer.init()
  await listener.init()

  listener.on('range', relayer.submit.bind(relayer))
}

main()

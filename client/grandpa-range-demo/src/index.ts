import Listener from './listener'
import Relayer from './relayer'

async function main() {
  const listener: Listener = new Listener()
  const relayer: Relayer = new Relayer()

  Listener.debug('🦅 remote endpoint', listener.endpoint)
  Relayer.debug('⚡ circuit endpoint', relayer.endpoint)
  Listener.debug('⛩️  gateway id', listener.gatewayId.toString())
  Listener.debug('🏔️  range size', listener.rangeSize)

  await Promise.all([listener.init(), relayer.init()])

  listener.on('range', relayer.submit.bind(relayer))
}

main()

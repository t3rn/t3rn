import Listener from './listener'
import Relayer from './relayer'

async function main() {
  const listener: Listener = new Listener()
  const relayer: Relayer = new Relayer()

  Listener.debug('endpoint', listener.endpoint)
  Relayer.debug('endpoint', relayer.endpoint)
  Listener.debug('gateway id', listener.gatewayId.toString())
  Listener.debug('range size', listener.rangeSize)

  await listener.init()

  listener.on('range', relayer.submit.bind(relayer))
}

main()

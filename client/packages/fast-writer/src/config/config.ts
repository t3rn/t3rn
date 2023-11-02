import * as dotenv from 'dotenv'
import envVar from 'env-var'
import { LogLevel } from './types'

dotenv.config()

const get = envVar.get

const config = () => ({
  prometheus: {
    port: get('PROMETHEUS_PORT').required().default(9133).asPortNumber(),
  },
  circuit: {
    rpc1: {
      ws: get('CIRCUIT_RPC1_WS').required().asString(),
    },
    rpc2: {
      ws: get('CIRCUIT_RPC2_WS').required().asString(),
    },
    signer: get('CIRCUIT_SIGNER_KEY').required().asString(),
  },
  log: {
    level: get('LOG_LEVEL')
      .required()
      .default(LogLevel.INFO)
      .asEnum(Object.values(LogLevel)),
    pretty: get('LOG_PRETTY').required().default('false').asBoolStrict(),
  },
  intervalSeconds: get('INTERVAL')
    .required()
    .default(12)
    .asInt(),
})

export default config()
export type Config = ReturnType<typeof config>

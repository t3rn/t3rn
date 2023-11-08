import * as dotenv from 'dotenv'
import envVar from 'env-var'

dotenv.config()

const get = envVar.get

export const config = () => ({
  t3rnConfigFile: get('T3RN_CONFIG_FILE')
    .default('.t3rn-config.json')
    .asString(),
  circuit: {
    rpc: {
      ws: get('CIRCUIT_WS_ENDPOINT').default('ws://127.0.0.1:9944').asString(),
      http: get('CIRCUIT_HTTP_ENDPOINT')
        .default('http://127.0.0.1:9944')
        .asString(),
    },
    signerKey: get('CIRCUIT_SIGNER_KEY').asString(),
  },
  targetChain: {
    beaconEndpoint: get('BEACON_ENDPOINT').required().asString(),
    executionEndpoint: get('EXECUTION_ENDPOINT').required().asString(),
    relayEndpoint: get('RELAY_ENDPOINT').required().asString(),
    lodestarEndpoint: get('LODESTAR_ENDPOINT').required().asString(),
  },
  eth: {
    consts: {
      epochsPerCommitteePeriod: get('ETHEREUM_EPOCHS_PER_COMMITTEE_PERIOD')
        .default(256)
        .asInt(),
      slotsPerEpoch: get('ETHEREUM_SLOTS_PER_EPOCH').default(32).asInt(),
    },
  },
})

export type Config = ReturnType<typeof config>

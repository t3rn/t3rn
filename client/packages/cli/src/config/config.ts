import * as dotenv from 'dotenv'
import envVar from 'env-var'

dotenv.config()

const get = envVar.get

export const config = () => ({
  t3rnConfigFile: get('T3RN_CONFIG_FILE').default('.t3rn-config.json').asString(),
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

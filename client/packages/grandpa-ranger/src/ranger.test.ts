import { GrandpaRanger } from './ranger'
import { cryptoWaitReady } from '@t3rn/sdk'
import * as collectModule from './collect'
import { Prometheus } from './prometheus'
import { Connection } from './connection'
import { logger } from './logging'

const grandpaRangerConfig = {
  circuit: {
    rpc1: {
      ws: 'ws://localhost',
      http: 'http://localhost',
    },
    rpc2: {
      ws: 'ws://localhost',
      http: 'http://localhost',
    },
  },
  target: {
    rpc1: {
      ws: 'ws://localhost',
    },
    rpc2: {
      ws: 'ws://localhost',
    },
  },
  rangeInterval: 0,
  targetGatewayId: 'roco',
  batches_max: 2,
  batching: true,
}

const generateRangeSpy = jest.spyOn(collectModule, 'generateRange')
const setCheckpointMetricsSpy = jest.spyOn(
  collectModule,
  'setCheckpointMetrics'
)
generateRangeSpy.mockImplementation(
  async (config, circuit, target, targetGatewayId) => {
    return []
  }
)
setCheckpointMetricsSpy.mockImplementation(
  async (config, circuit, target, prometheus) => {
    return
  }
)

jest.mock('./logging', () => {
  return {
    logger: {
      info: jest.fn(),
      error: jest.fn(),
      warn: jest.fn(),
      debug: jest.fn(),
    },
  }
})
jest.mock('@t3rn/sdk', () => {
  return {
    cryptoWaitReady: jest.fn().mockResolvedValue(true),
  }
})
jest.mock('./prometheus', () => {
  class MockPrometheus {
    rangeInterval = {
      inc: jest.fn(),
    }
    submissions = {
      inc: jest.fn(),
    }
    height = {
      set: jest.fn(),
    }
    txSize = {
      set: jest.fn(),
    }
  }
  return {
    Prometheus: MockPrometheus,
  }
})
jest.mock('./connection', () => {
  class MockConnection {
    isActive = true
    connect = jest.fn()
    sdk = {
      circuit: {
        tx: {
          signAndSendSafe: jest.fn(),
        },
      },
    }
  }
  return {
    Connection: MockConnection,
  }
})

describe('GrandpaRanger class', () => {
  let grandpaRanger

  beforeEach(async () => {
    grandpaRanger = new GrandpaRanger(grandpaRangerConfig)
    await grandpaRanger.start()
  })

  afterEach(async () => {
    grandpaRanger.stop()
    jest.useRealTimers()
  })

  it('Should connect to the clients', async () => {
    expect(grandpaRanger.circuit.connect).toHaveBeenCalled()
    expect(grandpaRanger.target.connect).toHaveBeenCalled()
  })

  it('Should start loops to submit ranges and collect metrics', async () => {
    expect(setCheckpointMetricsSpy.mock.calls.length).toBeGreaterThanOrEqual(1)
    expect(generateRangeSpy.mock.calls.length).toBeGreaterThanOrEqual(1)
  })

  it('Should slice batches when they are bigger than value in the config', async () => {
    const myMethodSpy = jest
      .spyOn(grandpaRanger, 'submitToCircuit')
      .mockImplementationOnce(() => Promise.resolve('a'))

    generateRangeSpy.mockImplementationOnce(
      async (config, circuit, target, targetGatewayId) => {
        return [
          { range: [] },
          { range: [] },
          { range: [] },
          { range: [] },
          { range: [] },
          { range: [] },
          { range: [] },
        ]
      }
    )

    await new Promise((resolve, _) => {
      grandpaRanger.collectAndSubmit(resolve)
    })

    expect(myMethodSpy).toHaveBeenCalledWith([{ range: [] }, { range: [] }])
  })
})

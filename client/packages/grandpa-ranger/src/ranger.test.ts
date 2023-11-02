import { GrandpaRanger } from './ranger'
import { cryptoWaitReady } from '@t3rn/sdk'
import * as collectModule from './collect'
import { Prometheus } from './prometheus'
import { Connection } from './connection'
import { logger } from './logging'

const mockedBatch = { range: [] }
const mockedHash = 'MOCKED_HASH'
const mockedConfig = {
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
generateRangeSpy.mockImplementation(async () => {
  return []
})
const setCheckpointMetricsSpy = jest.spyOn(
  collectModule,
  'setCheckpointMetrics',
)
setCheckpointMetricsSpy.mockImplementation(async () => {
  return
})

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
  const grandpaRanger = new GrandpaRanger(mockedConfig)

  beforeEach(async () => {
    await grandpaRanger.start()
  })

  afterEach(async () => {
    grandpaRanger.stop()
  })

  it('Should connect to the clients', async () => {
    expect(grandpaRanger.circuit.connect).toHaveBeenCalled()
    expect(grandpaRanger.target.connect).toHaveBeenCalled()
  })

  it('Should start loops to submit ranges and collect metrics', async () => {
    expect(setCheckpointMetricsSpy.mock.calls.length).toBeGreaterThanOrEqual(1)
    expect(generateRangeSpy.mock.calls.length).toBeGreaterThanOrEqual(1)
  })

  it('Should handle when the batch is empty', async () => {
    const resolve = jest.fn()
    await grandpaRanger.collectAndSubmit(resolve)

    expect(resolve).toHaveBeenCalled()
    expect(logger.warn).toHaveBeenCalledWith('No batches to submit')
  })

  it('Should handle batch slicing when the batch size exceeds the maximum', async () => {
    const submitToCircuitSpy = jest
      .spyOn(grandpaRanger, 'submitToCircuit')
      .mockImplementationOnce(() => Promise.resolve(mockedHash))

    generateRangeSpy.mockImplementationOnce(async () => {
      return [mockedBatch, mockedBatch, mockedBatch, mockedBatch, mockedBatch]
    })

    const resolve = jest.fn()
    await grandpaRanger.collectAndSubmit(resolve)

    expect(resolve).toHaveBeenCalled()
    expect(submitToCircuitSpy).toHaveBeenCalledWith([mockedBatch, mockedBatch])
  })

  it('Should handle batch slicing when the batch size is less than the maximum', async () => {
    const submitToCircuitSpy = jest
      .spyOn(grandpaRanger, 'submitToCircuit')
      .mockImplementationOnce(() => Promise.resolve(mockedHash))

    generateRangeSpy.mockImplementationOnce(async () => {
      return [mockedBatch]
    })

    const resolve = jest.fn()
    await grandpaRanger.collectAndSubmit(resolve)

    expect(resolve).toHaveBeenCalled()
    expect(submitToCircuitSpy).toHaveBeenCalledWith([mockedBatch])
  })

  it('Should handle errors during range generation', async () => {
    const mockedError = new Error('Error during range generation')
    generateRangeSpy.mockImplementationOnce(async () => {
      throw mockedError
    })

    const resolve = jest.fn()
    await grandpaRanger.collectAndSubmit(resolve)

    expect(resolve).toHaveBeenCalled()
    expect(logger.error).toHaveBeenCalledWith(mockedError)
  })

  it('Should handle errors during metric collection', async () => {
    const errorMessage = 'Error during metric collection'
    setCheckpointMetricsSpy.mockImplementationOnce(async () => {
      throw new Error(errorMessage)
    })

    await expect(setCheckpointMetricsSpy).rejects.toThrowError(errorMessage)
  })
})

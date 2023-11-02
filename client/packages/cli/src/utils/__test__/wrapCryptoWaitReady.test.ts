import { describe, expect, test } from '@jest/globals'
import type { MockedFunction } from 'jest-mock'
import { cryptoWaitReady } from '@t3rn/sdk'
import { wrapCryptoWaitReady } from '../fns.ts'

type MockedCryptoWaitReady = MockedFunction<typeof cryptoWaitReady>

jest.mock('@t3rn/sdk', () => ({
  cryptoWaitReady: jest.fn(),
}))

const mockedCryptoWaitReady = cryptoWaitReady as MockedCryptoWaitReady

describe('wrapCryptoWaitReady', () => {
  afterEach(() => {
    mockedCryptoWaitReady.mockClear()
  })

  test('should invoke callback function when crypto is ready', async () => {
    mockedCryptoWaitReady.mockImplementationOnce(() => Promise.resolve(true))

    const callback = jest.fn()
    await wrapCryptoWaitReady(callback)({ arg1: 'value' })
    expect(callback).toBeCalled()
  })

  test('should invoke callback function when crypto is ready', async () => {
    mockedCryptoWaitReady.mockImplementationOnce(() =>
      Promise.reject(new Error('Crypto is not ready')),
    )

    const callback = jest.fn()
    console.log = jest.fn()
    await wrapCryptoWaitReady(callback)({ arg1: 'value' })
    expect(callback).not.toBeCalled()
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining('ERROR'))
  })
})

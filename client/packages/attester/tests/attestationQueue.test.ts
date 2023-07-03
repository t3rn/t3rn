// Import necessary dependencies and modules
import { Attester } from '../src/attester'

jest.mock('../src/prometheus')
console.warn = jest.fn()

// Mock the dependencies
const mockCircuit = {
    client: {
        query: {
            attesters: {
                currentCommittee: jest.fn(),
            },
        },
    },
}

const mockPrometheus = {
    currentCommitteeMember: {
        set: jest.fn(),
    },
}

describe('queueTest', () => {
    let attester

    beforeEach(() => {
        // Create a new instance of Attester before each test
        const config = {
            circuit: {
                rpc1: 'mock1',
                rpc2: 'mock2',
            },
        }
        const keys = {
            substrate: {},
            ethereum: {
                privateKey:
                    '0x0123456789012345678901234567890123456789012345678901234567890123',
            },
            btc: {},
        }
        attester = new Attester(config, keys)
        Object.defineProperty(attester, 'prometheus', {
            get: jest.fn(() => mockPrometheus),
        })
        Object.defineProperty(attester, 'circuit', {
            get: jest.fn(() => mockCircuit),
        })
    })

    afterEach(() => {
        // Reset the mock implementation for each test
        jest.clearAllMocks()
    })

    it('should add items to queue', () => {
        const item = {
            messageHash: '0x123',
            targetId: '0x456',
            executionVendor: 'EVM',
        }

        expect(attester.q.length()).toBe(0)

        attester.q.push(item)

        expect(attester.q.length()).toBe(1)
    })

    it('should purge queue', async () => {
        const item = {
            messageHash: '0x123',
            targetId: '0x456',
            executionVendor: 'EVM',
        }

        attester.q.push(item)
        expect(attester.q.length()).toBe(1)

        attester.queuePurge()
        expect(attester.q.length()).toBe(0)
    })
})

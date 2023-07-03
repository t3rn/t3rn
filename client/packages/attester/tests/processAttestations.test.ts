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

describe('processAttestation', () => {
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

    it('should return true when attestationsDone has messageHash', async () => {
        attester.attestationsDone = ['0x1234']

        const result = attester.isAttestationDone('0x1234')

        expect(result).toBe(true)
    })

    it('should return false when attestationsDone doesnt have messageHash', async () => {
        attester.attestationsDone = []

        const result = await attester.isAttestationDone('0x1234')

        expect(result).toBe(false)
    })

    it('should return false when target is not allowed', async () => {
        attester.config.targetsAllowed = ['0x1234']

        const result = await attester.isTargetAllowed('0x5678')

        expect(result).toBe(false)
    })

    it('should return true when target is allowed', async () => {
        attester.config.targetsAllowed = ['0x1234']

        const result = await attester.isTargetAllowed('0x1234')

        expect(result).toBe(true)
    })
})

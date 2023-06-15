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

describe('attesterCommittee', () => {
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
            ethereum: {},
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

    it('should return true if the address is in the committee', () => {
        const committee = ['address1', 'address2', 'address3']
        const accountId = 'address2'

        attester.checkIsInCommittee(committee, accountId)

        // Assert that the result is true
        expect(attester.isInCurrentCommittee).toBe(true)
        expect(
            attester.prometheus.currentCommitteeMember.set
        ).toHaveBeenCalledWith(1)
    })

    it('should return false if the address is not in the committee', async () => {
        const committee = ['address1', 'address3']
        const accountId = 'address2'

        await attester.checkIsInCommittee(committee, accountId)

        // Assert that the result is false
        expect(attester.isInCurrentCommittee).toBe(false)
        expect(
            attester.prometheus.currentCommitteeMember.set
        ).toHaveBeenCalledWith(0)
    })

    // it('getCommittee should return array of strings', async () => {
    //     const committee = [
    //         'address1',
    //         'address2',
    //         'address3',
    //     ]
    //     const uint8ArrayArray = committee.map((address) => new Uint8Array(Buffer.from(address, 'utf8')))
    //     JSON.stringify(uint8ArrayArray).toJSON()
    //     attester.circuit.client.query.attesters.currentCommittee.mockResolvedValue(
    //         JSON.parse(JSON.stringify(committee))

    //     )
    //     // console.error()

    //     // Call the `getCommittee` function
    //     const result = await attester.getCommittee()

    //     // Assert that the result is false
    //     expect(result).toEqual(committee)
    // })
})

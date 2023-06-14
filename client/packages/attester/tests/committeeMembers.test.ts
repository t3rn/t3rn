// Import necessary dependencies and modules
import { Attester } from '../src/attester'

jest.mock('../src/prometheus')

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

// Describe the test suite for the `isInCommittee` function
describe('isInCommittee', () => {
    let attester

    beforeEach(() => {
        // Create a new instance of YourClass before each test
        const config = {
            circuit: {
                rpc1: 'mock1',
                rpc2: 'mock2',
            },
        }
        const keys = {
            substrate: {
            },
            ethereum: {
            },
            btc: {
            },
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

    it('should return true if the address is in the committee', async () => {
        attester.keys.substrate.addressId = 'address1'
        attester.circuit.client.query.attesters.currentCommittee.mockResolvedValue(
            [
                 'address1' ,
                 'address2' ,
                 'address3' ,
            ]
        )

        await attester.checkIsInCommittee()

        // Assert that the result is true
        expect(attester.isInCurrentCommittee).toBe(true)
        expect(
            attester.prometheus.currentCommitteeMember.set
        ).toHaveBeenCalledWith(1)
    })

    it('should return false if the address is not in the committee', async () => {
        attester.keys.substrate.addressId = 'address2'
        attester.circuit.client.query.attesters.currentCommittee.mockResolvedValue(
            ['address1' ,'address3' ]
        )

        // Call the `isInCommittee` function
        await attester.checkIsInCommittee()

        // Assert that the result is false
        expect(attester.isInCurrentCommittee).toBe(false)

        // Assert that the `currentCommitteeMember` metric was set to 0
        expect(
            attester.prometheus.currentCommitteeMember.set
        ).toHaveBeenCalledWith(0)
    })

})

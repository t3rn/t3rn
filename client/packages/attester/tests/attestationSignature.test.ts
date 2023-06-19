// Import necessary dependencies and modules
import { Attester } from '../src/attester'
import {
    ecsign,
    ecrecover,
    toBuffer,
    privateToPublic,
    hashPersonalMessage,
} from 'ethereumjs-util'

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
        tx: {
            attesters: {
                submitAttestation: jest.fn(),
            },
        },
    },
}

const mockPrometheus = {
    currentCommitteeMember: {
        set: jest.fn(),
    },
}

describe('generateAttestationTx', () => {
    let attester
    const messageHash =
        '0x58cd0ea9f78f115b381b29bc7edaab46f214968c05ff24b6b14474e4e47cfcdd'

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

    it('Should generate correct signature', async () => {
        const data = attester.generateAttestationTx(messageHash, 'sepl')

        expect(data.signature).toEqual(
            ''
        )
        const signatureBytes = toBuffer(data.signature)
        expect(signatureBytes.length).toEqual(65)
    })

    it('Should recover correct address with ecrrecover with sigObj', async () => {
        const publicKey = privateToPublic(
            toBuffer(attester.keys.ethereum.privateKey)
        )

        // Recover the signer's address
        const sigObj = ecsign(
            hashPersonalMessage(toBuffer(messageHash)),
            toBuffer(attester.keys.ethereum.privateKey)
        )
        const signedFrom = ecrecover(
            hashPersonalMessage(toBuffer(messageHash)),
            sigObj.v,
            sigObj.r,
            sigObj.s
        ).toString('hex')

        expect(signedFrom).toEqual(publicKey.toString('hex'))
    })
})

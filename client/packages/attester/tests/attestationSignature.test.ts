// Import necessary dependencies and modules
import { logger } from 'src/logging'
import { Attester } from '../src/attester'
import {
    ecsign,
    ecrecover,
    toBuffer,
    privateToPublic,
    hashPersonalMessage,
    keccak,
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
                    '0xe29186ac6e188f0c469f759c68fb2f07bac12ef1f38e111dfa21b66f2eee9858',
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
        const data = await attester.generateAttestationTx(messageHash, 'sepl')

        expect(data.signature).toEqual(
            '0x6493e3336802d9f01c2a0482d1d4a8b14a579c888a4eb18cd211fd88da9b62ec313e82c2a9db5c53618370d6baab60a06ca4f0d23995f280ed9e7401d1357d8f1b'
        )
        const signatureBytes = toBuffer(data.signature)
        expect(signatureBytes.length).toEqual(65)
    })

    it('Should recover correct address with ecrrecover with sigObj', async () => {
        const publicKey = privateToPublic(
            toBuffer(attester.keys.ethereum.privateKey)
        )

        const prefix = '\x19Ethereum Signed Message:\n32'
        const prefixBuffer = Buffer.from(prefix)
        const messageBuffer = toBuffer(messageHash)

        // Recover the signer's address
        const sigObj = ecsign(
            keccak(Buffer.concat([prefixBuffer, messageBuffer])),
            toBuffer(attester.keys.ethereum.privateKey)
        )
        const signedFrom = ecrecover(
            keccak(Buffer.concat([prefixBuffer, messageBuffer])),
            sigObj.v,
            sigObj.r,
            sigObj.s
        ).toString('hex')

        expect(signedFrom).toEqual(publicKey.toString('hex'))
    })
})

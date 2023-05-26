const ApiPromise = require('@polkadot/api').ApiPromise;

jest.mock('@polkadot/api', () => ({
    ApiPromise: {
        create: jest.fn().mockResolvedValue({
            tx: {
                attesters: {
                    registerAttester: jest.fn().mockReturnThis(),
                    deregisterAttester: jest.fn().mockReturnThis(),
                    submitAttestation: jest.fn().mockReturnThis(),
                    agreeToNewAttestationTarget: jest.fn().mockReturnThis(),
                    signAndSend: jest.fn().mockResolvedValue({}),
                },
            },
        }),
    },
}));

const {
    register_with_each_attester_key,
    deregister_with_each_attester_key,
    attest_with_each_attester_key,
    agree_to_target_with_each_attester_key,
} = require('./offchain-attester');

describe('register_with_each_attester_key', () => {
    it('registers with each attester key', async () => {
        const mockApi = await ApiPromise.create();
        const commission = 0.1;
        const nominateAmount = 100;

        await register_with_each_attester_key(mockApi, commission, nominateAmount);

        expect(mockApi.tx.attesters.registerAttester).toHaveBeenCalled();
        expect(mockApi.tx.attesters.signAndSend).toHaveBeenCalled();
    });
});

describe('deregister_with_each_attester_key', () => {
    it('registers with each attester key', async () => {
        const mockApi = await ApiPromise.create();

        await deregister_with_each_attester_key(mockApi);

        expect(mockApi.tx.attesters.deregisterAttester).toHaveBeenCalled();
        expect(mockApi.tx.attesters.signAndSend).toHaveBeenCalled();
    });
});

describe('attest_with_each_attester_key', () => {
    it('registers with each attester key', async () => {
        const mockApi = await ApiPromise.create();
        const executionVendor = 'EVM';
        const messageHash = '0x0000000000000000000000000000000000000000000000000000000000000000';
        const targetId = '0x00000000';
        await attest_with_each_attester_key(mockApi, targetId, messageHash, executionVendor);

        expect(mockApi.tx.attesters.submitAttestation).toHaveBeenCalled();
        expect(mockApi.tx.attesters.signAndSend).toHaveBeenCalled();
    });
});

describe('agree_to_target_with_each_attester_key', () => {
    it('registers with each attester key', async () => {
        const mockApi = await ApiPromise.create();
        const targetId = '0x00000000';
        await agree_to_target_with_each_attester_key(mockApi, targetId);

        expect(mockApi.tx.attesters.agreeToNewAttestationTarget).toHaveBeenCalled();
        expect(mockApi.tx.attesters.signAndSend).toHaveBeenCalled();
    });
});

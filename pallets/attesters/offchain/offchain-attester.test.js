const ApiPromise = require('@polkadot/api').ApiPromise;

jest.mock('@polkadot/api', () => ({
    ApiPromise: {
        create: jest.fn().mockResolvedValue({
            tx: {
                attesters: {
                    registerAttester: jest.fn().mockReturnThis(),
                    signAndSend: jest.fn().mockResolvedValue({}),
                },
            },
        }),
    },
}));

const { register_with_each_attester_key } = require('./offchain-attester');

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

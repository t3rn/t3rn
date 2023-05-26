const generate_random_private_keys = require('./generateKeys');

jest.mock('fs', () => ({
    writeFileSync: jest.fn(),
}));

// todo: align cryptoWaitReady between the versions.
describe.skip('generate_random_private_keys', () => {
    it('generates the specified number of keys', async () => {
        const count = 5;
        const keys = await generate_random_private_keys(count);

        expect(keys.length).toBe(count);
        keys.forEach((key) => {
            expect(key.substrate).toHaveProperty('accountId');
            expect(key.substrate).toHaveProperty('privateKey');
            expect(key.substrate).toHaveProperty('publicKey');

            expect(key.ethereum).toHaveProperty('privateKey');
            expect(key.ethereum).toHaveProperty('publicKey');
            expect(key.ethereum).toHaveProperty('address');

            expect(key.btc).toHaveProperty('privateKey');
            expect(key.btc).toHaveProperty('publicKey');
        });
    });
});

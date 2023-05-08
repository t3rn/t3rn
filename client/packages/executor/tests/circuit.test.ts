import { expect } from 'chai';
import { CircuitRelayer } from '../src/circuit/relayer';
import { Sdk } from "@t3rn/sdk";
import { BN } from "@polkadot/util"


// Include basic testing and the check for the methods in the next block
describe('Basic CircuitRelayer setup', () => {
    it('should be a class', () => {
        expect(CircuitRelayer).to.be.a('function');
    });

    it('should have a constructor', () => {
        expect(CircuitRelayer).to.have.property('constructor');
    });

    // TODO: Incorrect values, check.
    const sdk = new Sdk("ws://test", "0000");
    const cr = new CircuitRelayer(sdk);

    it('should have a method called "bidSfx"', () => {
        expect(cr).to.have.property('bidSfx');
    });
});


describe('Circuit relayer functionality', () => {
    // TODO: properly finish this
    it('should be able to build and submit an sfxBid to the circuit', () => {
        const sdk = new Sdk("ws://test", "0000");
        const cr = new CircuitRelayer(sdk);
        cr.bidSfx("0x123", new BN(100));
    })
});

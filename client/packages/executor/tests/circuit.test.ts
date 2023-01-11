import { expect } from 'chai';
import { CircuitRelayer } from '../src/circuit/relayer';
import { Sdk } from "@t3rn/sdk";
import { BN } from "@polkadot/util"


// Include basic testing and the check for the methods in the next block
describe('CircuitRelayer basic configuration', () => {
    it('should be a class', () => {
        expect(CircuitRelayer).to.be.a('function');
    });

    it('should have a constructor', () => {
        expect(CircuitRelayer).to.have.property('constructor');
    });

    it('should have a method called "bidSfx"', () => {
        expect(CircuitRelayer).to.have.property('bidSfx');
    });
});


describe('Circuit relayer funcitonality', () => {
    it('should be able to build and submit an sfxBid to the circuit', () => {
        const sdk = new Sdk();
        const cRelayer = new CircuitRelayer(sdk);
        cRelayer.bidSfx("0x123", new BN(100));
    })
});

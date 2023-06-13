import { expect } from 'chai';
import { StrategyEngine } from '../src/strategy/index';
import { BiddingEngine } from '../src/bidding/index';
import { SideEffect } from '../src/executionManager/sideEffect';

describe('Basic StrategyEngine setup', () => {
    it('should be a class', () => {
        expect(StrategyEngine).to.be.a('function');
    });

    it('should have a constructor', () => {
        expect(StrategyEngine).to.have.property('constructor');
    });

    const se = new StrategyEngine();

    it('should have a method called "getMinProfitUsd"', () => {
        expect(se).to.have.property('getMinProfitUsd');
    });
})

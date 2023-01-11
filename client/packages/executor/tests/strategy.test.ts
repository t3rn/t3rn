import { expect } from 'chai';
import { StrategyEngine } from '../src/strategy/index';
import { BiddingEngine } from '../src/bidding/index';
import { SideEffect } from '../src/executionManager/sideEffect';

// TODO: function to create a side effect with random values

describe('StrategyEngine', () => {
    it('should be a class', () => {
        expect(StrategyEngine).to.be.a('function');
    });

    it('should have a constructor', () => {
        expect(StrategyEngine).to.have.property('constructor');
    });

    it('should have a method called "getMinProfitUsd"', () => {
        expect(StrategyEngine).to.have.property('getMinProfitUsd');
    });

    it('should have a method called "getMinProfitPercentage"', () => {
        expect(StrategyEngine).to.have.property('getMinProfitPercentage');
    });

    it('should return 0 when no profit is set', () => {
        const strategyEng = new StrategyEngine();
        const biddingEng = new BiddingEngine();
        const sfx = new SideEffect()
        expect(strategyEng.getMinProfitUsd()).to.be.equal(0);
        expect(strategyEng.getMinProfitPercentage()).to.be.equal(0);
    });
})

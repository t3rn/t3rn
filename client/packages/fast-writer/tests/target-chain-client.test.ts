import { expect } from 'chai'
import config from '../src/config/config.js'
import { TargetChainClient } from '../src/services/target-chain/client.js'

describe('request', () => {
  const targetChainClient = new TargetChainClient(config)

  describe('slotToCommitteePeriod', () => {
    it('should calculate committee period 400 for slot 3284991 (last slot for this period)', () => {
      expect(targetChainClient.slotToCommitteePeriod(3284991)).to.equal(400)
    })

    it('should calculate committee period 401 for slot 3284992 (first slot for this period)', () => {
      expect(targetChainClient.slotToCommitteePeriod(3284992)).to.equal(401)
    })

    it('should calculate committee period 401 for slot 3293152', () => {
      expect(targetChainClient.slotToCommitteePeriod(3293152)).to.equal(401)
    })

    it('should calculate committee period 401 for slot 3293183 (last slot for this period)', () => {
      expect(targetChainClient.slotToCommitteePeriod(3293183)).to.equal(401)
    })

    it('should calculate committee period 402 for slot 3293184 (first slot for this period)', () => {
      expect(targetChainClient.slotToCommitteePeriod(3293184)).to.equal(402)
    })
  })

  describe('committeePeriodToSlot', () => {
    it('should calculate slot 3284991 for committee period 400', () => {
      expect(targetChainClient.committeePeriodToSlot(400)).to.equal(3284960)
    })

    it('should calculate slot 3293183 for committee period 401', () => {
      expect(targetChainClient.committeePeriodToSlot(401)).to.equal(3293152)
    })

    it('should calculate slot 3301375 for committee period 402', () => {
      expect(targetChainClient.committeePeriodToSlot(402)).to.equal(3301344)
    })
  })

  describe('slotToEpoch', () => {
    it('should calculate slot 3301375 for epoch 103167', () => {
      expect(targetChainClient.slotToEpoch(3301375)).to.equal(103167)
    })

    it('should calculate slot 3293184 for epoch 102912', () => {
      expect(targetChainClient.slotToEpoch(3293184)).to.equal(102912)
    })

    it('should calculate slot 3293183 for epoch 102911', () => {
      expect(targetChainClient.slotToEpoch(3293183)).to.equal(102911)
    })
  })
})

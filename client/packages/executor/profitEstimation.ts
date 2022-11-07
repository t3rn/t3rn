import { SideEffect } from "./circuit/executions/sideEffect";
import {TransactionType} from "./circuit/executions/sideEffect";
const BN = require("bn.js");

const ROCOCO_PRICE = 100;
const T3RN_PRICE = 10;

export class ProfitEstimation {
	netProfit: number = 5 // the net profit must be 5. This does not include any transaction fees

	calculateProfitability(sideEffect: SideEffect): number {
		switch(sideEffect.action){
			case TransactionType.Transfer: {
				const cost = new BN(sideEffect.arguments[2].split('0x')[1], 16, "le").toNumber()
				const reward = sideEffect.hasInsurance ? 10 : 0; // ToDo use real value once optionalInsurance is refactored
				return reward - cost
			}
		}
	}

	shouldExecute(sideEffect: SideEffect): boolean {
		return this.calculateProfitability(sideEffect) >= this.netProfit
	}
}
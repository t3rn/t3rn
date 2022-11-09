import {CoingeckoPricing} from "./coingecko";
import {BehaviorSubject} from "rxjs";


export class PriceEngine {

	coingecko: CoingeckoPricing

	constructor() {
		this.coingecko = new CoingeckoPricing();
	}

	getAssetPrice(assetId: string): BehaviorSubject<number> {
		return this.coingecko.prices[assetId]
	}

}
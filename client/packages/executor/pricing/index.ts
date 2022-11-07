import {CoingeckoPricing} from "./coingecko";


export class PriceEngine {

	coingecko: CoingeckoPricing

	constructor() {
		this.coingecko = new CoingeckoPricing();
	}

	// In USD
	getQuote(assetId: string, amount: number) {
		return this.coingecko.prices[assetId] * amount
	}

}
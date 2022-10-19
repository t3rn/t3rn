import config from "../config/config"
const axios = require('axios');

export class CoingeckoPricing {

	endpoint: string;
	assets: {
		[assetTicker: string]: string
	} = {};

	prices: {
		[assetTicker: string]: number
	} = {};

	constructor() {
		this.endpoint = config.priceSource.coingecko.endpoint;
		this.getTrackingAssets()
		this.getAssetPrices()
	}

	getTrackingAssets() {
		let keys = Object.keys(config.assets);
		for(let i = 0; i < keys.length; i++) {
			if(config.assets[keys[i]].priceSource === "coingecko") {
				this.assets[keys[i]] = config.assets[keys[i]].id;
			}
		}
	}

	async getAssetPrices() {
		const ids = Object.keys(this.assets);
		for(let i = 0; i < ids.length; i++) {
			await axios.get(
				config.priceSource.coingecko.endpoint + this.assets[ids[i]] + "?localization=false&tickers=false&community_data=false&developer_data=false&sparkline=false"
			)
			.then(res => {
				const price = parseFloat(res.data.market_data.current_price["usd"])
				this.prices[ids[i]] = price
			})
		}
		console.log("Prices:", this.prices);
	}




}
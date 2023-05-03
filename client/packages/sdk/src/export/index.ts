import {SubmittableExtrinsic} from "@polkadot/api/promise/types";

interface ExtrinsicParam {
	name: string,
	rustType: string,
	encoded: string,
	decoded: any
}

export class ExtrinsicExport {
	tx: SubmittableExtrinsic
	section: string
	method: string
	args: ExtrinsicParam[] = [];
	submissionHeight: number



  	constructor(tx: SubmittableExtrinsic) {
		this.tx = tx
		this.handleParams()
  	}

  	async handleParams() {
		console.log("exporting transaction:")
		// console.log(this.tx.method.toHuman(true))
		const decoded = this.tx.method.toHuman(true)
		// @ts-ignore
		this.section = decoded.section.toString()
		// @ts-ignore
		this.method = decoded.method.toString()

		// @ts-ignore
		this.decodedArgs = this.tx.method.toPrimitive().args


		const args = this.tx.method.args;
		// @ts-ignore
		const paramDesc = this.tx.method._meta.toJSON().args
		//
		for(let i = 0; i < args.length; i++) {
			const param: ExtrinsicParam = {
				name: paramDesc[i].name,
				rustType: paramDesc[i].typeName,
				encoded: args[i].toHex(),
				decoded: args[i].toPrimitive()
			}
			this.args.push(param)
		}

		console.log("section:", this.section)
		console.log("method:", this.method)
		console.log("args:", this.args)

  	}
}
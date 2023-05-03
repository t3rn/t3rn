import {SubmittableExtrinsic} from "@polkadot/api/promise/types";
import * as fs from "fs";
import * as process from "process";
require('dotenv').config()

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
	error: string;

	constructor(tx: SubmittableExtrinsic) {
		this.tx = tx
		this.handleParams()
	}

	handleParams() {
		const decoded = this.tx.method.toHuman(true)
		// @ts-ignore
		this.section = decoded.section.toString()
		// @ts-ignore
		this.method = decoded.method.toString()

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
	}
	addErr(dispatchError: any) {
		this.error = dispatchError.toHex()
		this.toJSON()
	}

	addSubmissionHeight(height: number) {
		this.submissionHeight = height
		this.toJSON()
	}

	toJSON() {
		const json = JSON.stringify({
			section: this.section,
			method: this.method,
			args: this.args,
			submissionHeight: this.submissionHeight,
			error: this.error
		}, null, 4);

		const fileName = `/${this.submissionHeight}_${this.section}_${this.method}.json`
		const path = process.env.EXPORT_PATH || "./exports"

		fs.writeFileSync(`${path}${fileName}`, json);

	}
}
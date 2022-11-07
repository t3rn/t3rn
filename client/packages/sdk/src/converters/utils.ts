

export const toU8aId = (targetId: string | number[]): number[] => {
	let res;
	if(typeof targetId === 'string') {
		res = new TextEncoder().encode(targetId);
	} else {
		res = targetId
	}

	if(res.length !== 4) {
		throw new Error("Invalid Id!")
	}

	return res
}

export const toIdString = (targetId: number[] | string): string => {
	if(typeof targetId === 'string') {
		return targetId
	} else {
		return new TextDecoder().decode(Buffer.from(targetId));
	}
}

// encodes data for exporting. We export in encoded and human format.
// Encoded: We use for seeding portal rust tests
// Human: Debugging those tests and viewing data
export const encodeExport = (data: any, transactionType: string, submissionHeight: string) => {
    if(Array.isArray(data)) {
        return data.map(entry => iterateEncode(entry, transactionType, submissionHeight))
    } else {
        return iterateEncode(data, transactionType, submissionHeight)
    }
}

const iterateEncode = (data: any, transactionType: string, submissionHeight: string) => {
    let keys = Object.keys(data);
    let result = {};
    if(keys.includes("initialU8aLength")) { // this is a polkadot/apiPromise object
        return {
            data: data.toHuman(),
            transaction_type: transactionType,
            encoded_data: data.toHex().substring(2)
        }
    } else {
        for(let i = 0; i < keys.length; i++) {
            result['encoded_' + toSnakeCase(keys[i])] = data[keys[i]].toHex().substring(2)
            result[toSnakeCase(keys[i])] = data[keys[i]].toHuman()
        }
        result['transaction_type'] = transactionType;
        result['submission_height'] = submissionHeight
        return result;
    }
}

const toSnakeCase = str =>
  str &&
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    .map(x => x.toLowerCase())
    .jo
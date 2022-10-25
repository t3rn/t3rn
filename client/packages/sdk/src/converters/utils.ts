

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
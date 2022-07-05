const axios = require('axios').default;
import { decodeFinalityProof } from "../util";

export const fetchMissingAuthorityUpdateJustifications = async (gatewayBlock: number, api: any, endpoint: string) => {
    const authorityUpdateBlocks: number[] = await fetchAuthorityUpdateBlocks(endpoint);
    const missingAuthBlocks = authorityUpdateBlocks.filter(block => block >= gatewayBlock)
    missingAuthBlocks.sort(); // need to be asc order
    return collectProofData(missingAuthBlocks, api)
}

const fetchAuthorityUpdateBlocks = async (endpoint: string) => {
    return axios.post(endpoint + '/api/scan/events', {
            row: 100, // on rococo this ~4 days worth of justifications. Depends on AuthorityUpdate frequency (polkadot its more like 10 days)
            page: 1,
            module: "grandpa",
            call: "newauthorities"
        },
        {
            headers: {
                'content-type': 'text/json'
            }
        }
    )
    .then(function (response) {
        return response.data.data.events.map(entry => entry.block_num)
    })
    .catch(function (error) {
        console.log(error);
    });
}

const collectProofData = async (blockNumbers: number[], api: any) => {
    let res: any[] = [];
    for(let i = 0; i < blockNumbers.length; i++) {
        const finalityProof = await api.rpc.grandpa.proveFinality(blockNumbers[i]);
        const {blockHash, justification} = decodeFinalityProof((finalityProof))
        const header = await api.rpc.chain.getHeader(blockHash);
        res.push({header, justification})
    }
    return res;
}
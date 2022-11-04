import { encodings } from "@t3rn/sdk";
import {fetchBestFinalizedHash, fetchLatestPossibleParachainHeader, getStorageProof} from "../../utils/substrate";
const axios = require('axios').default;

export const submitParachainHeaders = async (circuit: any, target: any, gatewayData: any) => {
    return generateParachainProof(circuit, target, gatewayData)
}

export const submitRelaychainHeaders = async (circuit: any, target: any, gatewayId: string) => {
    const from = (await fetchGatewayHeight(gatewayId, circuit)) + 1;
    console.log("Latest on Gateway:", from - 1);
    const to = await fetchCurrentHeight(target);
    console.log("Latest on Target: ", to.toNumber());
    return generateBatchProof(circuit, target, gatewayId, from, to)
}

const generateBatchProof = async (circuit: any, target: any, gatewayId: string, from: number, to: number) => {
    let transactionArguments: any[] = [];
    while(from < to) {
        // get finalityProof element of epoch that contains block #from
        const finalityProof = await target.rpc.grandpa.proveFinality(from)
        // decode finality proof
        let { justification, headers } = encodings.substrate.decoders.finalityProofDecode(finalityProof)
        let signed_header = headers.pop()

        // query from header again, as its not part of the proof, and concat
        headers = [await getHeader(target, from), ...headers]
        let range = circuit.createType("Vec<Header>", headers)

        console.log("Batch:")
        console.log(`Range: From #${range[0].number.toNumber()} to #${range[range.length - 1].number.toNumber()}`)
        console.log("__________________________________________________________________")

        const relaychainHeaderData = circuit.createType("RelaychainHeaderData<Header>", {
            signed_header,
            range,
            justification: encodings.substrate.decoders.justificationDecode(justification)
        })

        //push to transaction queue
        transactionArguments.push({gatewayId: circuit.createType("ChainId", gatewayId), data: relaychainHeaderData})
        from = parseInt(signed_header.number.toJSON()) + 1
    }
    return transactionArguments;
}

const generateParachainProof = async (circuit: any, target: any, gatewayData: any) => {
    const latestRelayChainHeader = await fetchBestFinalizedHash(circuit, gatewayData.registrationData.parachain.relayChainId)
    const parachainHeader: any = await fetchLatestPossibleParachainHeader(
        gatewayData.relaychainRpc,
        latestRelayChainHeader.toJSON(),
        gatewayData.registrationData.parachain.id
    )
    const decodedParachainHeader = encodings.substrate.decoders.headerDecode(parachainHeader.toJSON())
    const parachainHeightCircuit = await fetchGatewayHeight(gatewayData.id, circuit)
    console.log("Latest Para Finalized Height:", parachainHeightCircuit)
    console.log("Newest potential header:", decodedParachainHeader.number.toNumber())
    let headers = await collectHeaderRange(target, parachainHeightCircuit + 1, decodedParachainHeader.number.toNumber() - 1) // the new highest header is in the proof

    const proof = await getStorageProof(gatewayData.relaychainRpc, latestRelayChainHeader.toJSON(), gatewayData.registrationData.parachain.id)

    const parachainHeaderData = circuit.createType("ParachainHeaderData<Header>", {
        relay_block_hash: latestRelayChainHeader.toJSON(),
        range: headers,
        proof: {
            trieNodes: proof.toJSON().proof
        }
    })
    return [{gatewayId: circuit.createType("ChainId", gatewayData.id), data: parachainHeaderData}]
}

const collectHeaderRange = async (target: any, from: number, to: number) => {
    let headers: any[] = [];
    while(from <= to) {
        headers.push(
            (await target.rpc.chain.getHeader(
                await target.rpc.chain.getBlockHash(from)
            )).toJSON()
        )
        console.log("fetched #", from)

        from += 1
    }
    return headers;
}

// ToDo this should be replaced for portal RPC call once #380 is closed
const fetchGatewayHeight = async (gatewayId: any, circuit: any) =>  {
    const hash = await circuit.query.rococoBridge.bestFinalizedMap(gatewayId);
    const height = await circuit.query.rococoBridge.multiImportedHeaders(gatewayId, hash.toJSON());
    if (height.toJSON()) {
        return height.toJSON().number
    } else {
        console.log("Gateway not Registered!")
        process.exit(1)
    }
}

const fetchCurrentHeight = async (target: any) => {
    const header = await target.rpc.chain.getHeader(
        await target.rpc.chain.getFinalizedHead()
    );

    return header.number
}

const getHeader = async (target: any, height: number) => {
    return (await target.rpc.chain.getHeader(
        await target.rpc.chain.getBlockHash(height)
    )).toJSON()
}

//for registrations we want to get the justification cotaining the latest authoritySetUpdate, as we can be sure that all authorties are included.
const fetchHeaders = async (gatewayData: any, page: number) => {
    return axios.post(gatewayData.subscan + '/api/scan/blocks', {
            row: 100,
            page,
        },
        {
            headers: {
                'content-type': 'text/json'
            }
        }
    )
    .then(function (response) {
        console.log(response.data.data)
        // return response.data.data.events.map(entry => entry.block_num)
    })
}
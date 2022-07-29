import {decodeHeader, decodeJustification, decodeFinalityProof} from "../../utils/decoder";
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
        const finalityProof = await target.rpc.grandpa.proveFinality(from)
        let {latestBlockHash, justification, headers} = decodeFinalityProof(finalityProof)
        let signed_header = headers.pop()
        headers.reverse() // we need to submit backwards
        headers.push(await getHeader(target, from)) // we need to fetch the target header seperatly
        let range = circuit.createType("Vec<Header>", headers)
        console.log("Batch:")
        console.log(`Range: From #${range[range.length - 1].number.toNumber()} to #${range[0].number.toNumber()}`)
        console.log("Signed Header:", signed_header.toHuman())
        console.log("Justification:", decodeJustification(justification).toHuman())
        console.log("__________________________________________________________________")
        const relaychainHeaderData = circuit.createType("RelaychainHeaderData<Header>", {
            signed_header,
            range,
            justification: decodeJustification(justification)
        })
        transactionArguments.push(relaychainHeaderData)
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
    const decodedParachainHeader = decodeHeader(parachainHeader.toJSON())
    console.log(decodedParachainHeader.hash.toHex())

    const parachainHeightCircuit = await fetchGatewayHeight(gatewayData.id, circuit)
    console.log("Latest Para Finalized Height:", parachainHeightCircuit)
    console.log("Newest potential header:", decodedParachainHeader.number.toNumber())
    let headers = await collectHeaderRange(target, parachainHeightCircuit + 1, decodedParachainHeader.number.toNumber() - 1) // the new highest header is in the proof

    const proof = await getStorageProof(gatewayData.relaychainRpc, latestRelayChainHeader.toJSON(), gatewayData.registrationData.parachain.id)

    console.log(proof.toHuman())
    //
    return [circuit.createType("ParachainHeaderData<Header>", {
        relay_block_hash: latestRelayChainHeader.toJSON(),
        range: headers.reverse(),
        proof: {
            trieNodes: proof.toJSON().proof
        }
    })]
    // return []


    // get relaychain latest finalized from circuit -> DONE
    // get parachain header at that block from relaychain -> signed_block
    // get parachain latest finalized from circuit
    // gather header range somehow
    // generate read proof for signed_header



}

const collectHeaderRange = async (target: any, from: number, to: number) => {
    let headers: any[] = [];
    while(from <= to) {
        headers.push(
            await target.rpc.chain.getHeader(
                await target.rpc.chain.getBlockHash(from)
            )
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
    return target.rpc.chain.getHeader(
        await target.rpc.chain.getBlockHash(height)
    )
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
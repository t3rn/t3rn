import createDebug from "debug"
import Web3 from "web3";
import BN from "bn.js"
import { config } from "./config";
import { EventEmitter } from "events"

export default class BSCListener extends EventEmitter {
    // static debug = createDebug("relaychain-listener")

    instance: Web3
    gatewayId: string
    headers: any[] = []
    latestEpoch: number = 0


    setup() {
        this.instance = new Web3(config.rpc)
    }

    initListener() {
        this.instance.eth.subscribe('newBlockHeaders', (err, header) => {
            if(err) console.error(err);

            this.appendHeader(header)

            // we dont want to retry fore each new block
            if(this.headers.length % config.rangeSize === 0) {
                console.log("Submitting!")
                this.submitHeader(header)
            }
        })
    }

    appendHeader(header: any) {
        if (
            this.headers.length === 0 ||
            (this.headers[this.headers.length - 1].number &&
                this.headers[this.headers.length - 1].number + 1 ===
                header.number)
        ) {
            console.log("Added #" + header.number)
            this.headers.push(header)
        } else {
            // if dup/uncle unset the header so that we query the correct one later
            const idx = this.headers.findIndex(h => {
                const headerNumber = typeof h === "number" ? h : h.number.toNumber()
                return headerNumber === header.number.toNumber()
            })
            console.log("found duplicate")
            if (idx !== -1) {
                this.headers[idx] = header.number.toNumber()
            }
        }

        if(header.number % 200 == 0) {
            console.log("Detected Epoch Block!")
            this.submitHeader(header); // Epoch blocks contain validator set update, so we must 'triage it'
        }
    }

    submitHeader(header: any) {
        let index = this.headers.indexOf(header)
        console.log("Submitting Header:", header.hash);
        const headerToAdd = this.headers[index];
        this.emit("SubmitHeader", {
            headerIndex: index, // important for submitting range
            header: headerToAdd,
        })
    }

    getHeaderRange(index: number) {
        const anchorHeader = this.headers[index]
        console.log("InQueue:", this.headers.length)
        const range = this.headers.splice(0, index).reverse()
        return {anchorHeader, range} 
    }

}
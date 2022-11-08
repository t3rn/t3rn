import createDebug from "debug"
import {Execution} from "./execution";
import {SideEffect} from "./sideEffect";

// A type used for storing the different SideEffects throughout their respective life-cycle.
// Please note that waitingForInsurance and readyToExecute are only used to track the progress. The actual logic is handeled in the execution
type Queue = {
    gateways: {
        blockHeight: number
		isBidding: string[],
		isExecuting: string[],
        // Executed sfx and their respective execution block.
        isConfirming: {
            [block: number]: string[]
        },
		complete: string[]
    }
}


export class ExecutionManager {
	static debug = createDebug("execution-manager")

	// we map the current state in the queue
	queue: Queue = <Queue>{}
    // maps xtxId to Execution instance
    xtx: {
        [id: string]: Execution
    } = {}
    // a lookup mapping, to find a sfx xtxId
    sfxToXtx: {
        [sfxId: string]: string
    } = {};


	// adds gateways on startup
    addGateway(id: string) {
        this.queue[id] = {
            blockHeight: 0,
            waitingForInsurance: [],
            readyToExecute: [],
            readyToConfirm: {},
        }

		console.log("added gateway", id)
		console.log(this.queue)
    }

}
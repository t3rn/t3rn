require('dotenv').config()
import CircuitListener from "./circuit/listener";
import CircuitRelayer from "./circuit/relayer";
import SubstrateRelayer from "./gateways/substrate/relayer";
import config from "./config.json"
import { SideEffectStateManager } from "./utils/types";

class Executor {
    circuitListener: CircuitListener;
    circuitRelayer: CircuitRelayer;
    gatewayInstances: any;
    currentlyRunning = {};

    constructor() {
        this.circuitListener = new CircuitListener();
        this.circuitRelayer = new CircuitRelayer();
    }

    async setup() {
        await this.circuitListener.setup(config.circuit.rpc)
        await this.circuitRelayer.setup(config.circuit.rpc)
        await this.circuitListener.start()
        await this.initializeGateways()
        console.log("Main - Instances Initialized")
    }

    async initializeGateways() {
        let gatewayInstances = {};
        for(let i = 0; i < config.gateways.length; i++) {
            const entry = config.gateways[i]
            if(entry.type === "substrate") {
                let instance = new SubstrateRelayer();
                await instance.setup(entry.rpc, entry.name)

                instance.on("txFinalized", data => {
                    this.handleSideEffectExecution(data)
                })

                gatewayInstances[entry.id] = instance;
            }
        }
        this.gatewayInstances = gatewayInstances;
    }

    async start() {
        this.circuitListener.on('NewSideEffect', (data) => {
            this.sideEffectRouter(data)
        })
    }

    async sideEffectRouter(sideEffectStateManager: SideEffectStateManager) {
        // store side effect for confirm step
        this.currentlyRunning[sideEffectStateManager.getId()] = sideEffectStateManager;
        this.gatewayInstances[sideEffectStateManager.getTarget()].executeTx(sideEffectStateManager)
    }

    async handleSideEffectExecution(xtxId: string) {

        console.log(this.currentlyRunning[xtxId])
        // console.log(this.currentlyRunning[sideEffectStateManager.getId()])
        // let sideEffect = this.addCompletionData(xtxId, completionData);


        // this.circuitRelayer.confirmSideEffect(sideEffect)
    }
}

(async () => {
    let exec = new Executor();
    await exec.setup()
    exec.start()
})()
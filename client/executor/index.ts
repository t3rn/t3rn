require('dotenv').config()
import CircuitListener from "./circuit/listener";
import CircuitRelayer from "./circuit/relayer";
import SubstrateRelayer from "./gateways/substrate/relayer";
import config from "./config.json"
import { colors } from "./utils/helpers";
import { SideEffectStateManager } from "./utils/types";
import chalk from 'chalk';

class Executor {
    circuitListener: CircuitListener;
    circuitRelayer: CircuitRelayer;
    gatewayInstances: any;
    currentlyRunning = {};
    color: string;

    constructor() {
        this.circuitListener = new CircuitListener();
        this.circuitRelayer = new CircuitRelayer();
        this.color = colors[0];
    }

    log(msg: string) {
        console.log(chalk[this.color]("index.ts - "), msg)
    }

    async setup() {
        await this.circuitListener.setup(config.circuit.rpc)
        await this.circuitRelayer.setup(config.circuit.rpc, colors[1])
        await this.circuitListener.start()
        await this.initializeGateways()
        this.log("Components Initialzed")
    }

    async initializeGateways() {
        let gatewayInstances = {};
        for(let i = 0; i < config.gateways.length; i++) {
            const entry = config.gateways[i]
            if(entry.type === "substrate") {
                let instance = new SubstrateRelayer();
                await instance.setup(entry.rpc, entry.name, colors[i + 2])

                instance.on("txFinalized", data => {
                    this.handleSideEffectExecution(data)
                })

                instance.on("SideEffectConfirmed", data => {
                    this.handleCompletion(data);
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
        this.circuitRelayer.confirmSideEffect(this.currentlyRunning[xtxId])
    }

    async handleCompletion(xtxId: string) {
        delete this.currentlyRunning[xtxId];
    }
}

(async () => {
    let exec = new Executor();
    await exec.setup()
    exec.start()
})()
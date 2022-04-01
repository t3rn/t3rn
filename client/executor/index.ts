require('dotenv').config()
import CircuitListener from "./circuit/listener";
import CircuitRelayer from "./circuit/relayer";
import SubstrateRelayer from "./gateways/substrate/relayer";
import config from "./config.json"

import { deconstruct, SideEffect } from "./utils/sideEffectInterfaces";

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
    }

    async initializeGateways() {
        let gatewayInstances = {};
        for(let i = 0; i < config.gateways.length; i++) {
            const entry = config.gateways[i]
            if(entry.type === "substrate") {
                let instance = new SubstrateRelayer();
                await instance.setup(entry.rpc)

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

    async sideEffectRouter(eventData: any) {
        let sideEffect = deconstruct(eventData);
        // store side effect for confirm step
        this.currentlyRunning[sideEffect.xtxId.toHuman()] = sideEffect;
        this.gatewayInstances[sideEffect.target.toHuman()].handleTx(sideEffect)
    }

    async handleSideEffectExecution(data: any) {
        const sideEffect = this.currentlyRunning[data.xtxId]
        this.circuitRelayer.confirmSideEffect(sideEffect, data)

    }

    

}

(async () => {
    let exec = new Executor();
    await exec.setup()
    exec.start()
})()
import CircuitListener from "./circuit/listener";
import CircuitRelayer from "./circuit/relayer";
import SubstrateRelayer from "./gateways/substrate/relayer";
import config from "./config.json"
// import { colors } from "./utils/helpers";
import { SideEffect } from "./utils/types";
import { ExecutionManager } from "./utils/executionManager";
import chalk from 'chalk';
import "dotenv/config"

class InstanceManager {
    circuitListener: CircuitListener;
    circuitRelayer: CircuitRelayer;
    executionManager: ExecutionManager;

    instances: {
        [id: string]: SubstrateRelayer
    } = {};
    // color: string;

    constructor() {
        this.circuitListener = new CircuitListener();
        this.circuitRelayer = new CircuitRelayer();
        this.executionManager = new ExecutionManager();

        // this.color = colors[0];
    }

    log(msg: string) {
        // console.log(chalk[this.color]("index.ts - "), msg)
        console.log("index.ts - ", msg)
    }

    async setup() {
        await this.circuitListener.setup(config.circuit.rpc)
        await this.circuitRelayer.setup(config.circuit.rpc, )//colors[1])
        await this.circuitListener.start()
        await this.initializeGateways()
        this.log("Components Initialzed")
    }

    async initializeGateways() {
        for(let i = 0; i < config.gateways.length; i++) {
            const entry = config.gateways[i]
            if(entry.type === "substrate") {
                let instance = new SubstrateRelayer();
                await instance.setup(entry.rpc, entry.name, )//colors[i + 2])


                instance.on("SideEffectExecuted", (id: string) => {
                    console.log("SideEffectExecuted")
                    this.executionManager.sideEffectExecuted(id)
                })

                // setup in executionManager
                this.executionManager.addGateway(entry.id);
                // store relayer instance locally
                this.instances[entry.id] = instance;
            }
        }
    }

    async initializeEventListeners() {
        this.circuitListener.on('NewSideEffect', (data: SideEffect) => {
            console.log('NewSideEffect')
            this.executionManager.addSideEffect(data);
        })

        this.executionManager.on('ExecuteSideEffect', sideEffect => {
            console.log('ExecuteSideEffect')
            this.instances[sideEffect.getTarget()].executeTx(sideEffect)
        })

        this.executionManager.on("ConfirmSideEffects", (sideEffects: SideEffect[]) => {
            console.log("ConfirmSideEffects")
            this.circuitRelayer.confirmSideEffects(sideEffects)
        })

        this.circuitRelayer.on("SideEffectConfirmed", (id: string) => {
            console.log("SideEffectConfirmed")
            this.executionManager.finalize(id);
        })

        this.circuitListener.on('NewHeaderRangeAvailable', (data) => {
            console.log('NewHeaderRangeAvailable')
            this.executionManager.updateGatewayHeight(data.gatewayId, data.height);
        })

        
    }
}

(async () => {
    let exec = new InstanceManager();
    await exec.setup()
    exec.initializeEventListeners()
})()
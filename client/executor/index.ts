require('dotenv').config()
import CircuitListener from "./circuit/listener";
import CircuitRelayer from "./circuit/relayer";
import SubstrateRelayer from "./gateways/substrate/relayer";
import config from "./config.json"
import { colors } from "./utils/helpers";
import { SideEffectStateManager } from "./utils/types";
import { ExecutionManager } from "./utils/executionManager";
import chalk from 'chalk';

class InstanceManager extends ExecutionManager {
    circuitListener: CircuitListener;
    circuitRelayer: CircuitRelayer;

    instances: {
        [id: string]: SubstrateRelayer
    } = {};
    color: string;

    constructor() {
        super();
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
        for(let i = 0; i < config.gateways.length; i++) {
            const entry = config.gateways[i]
            if(entry.type === "substrate") {
                let instance = new SubstrateRelayer();
                await instance.setup(entry.rpc, entry.name, colors[i + 2])

                instance.on("SideEffectExecuted", (id: string) => {
                    console.log("SideEffectExecuted")
                    this.executedSideEffect(id)
                    this.circuitRelayer.confirmSideEffect(this.sideEffects[id])
                })

            
                this.addGateway(entry.id);

                this.instances[entry.id] = instance;
            }
        }
    }

    async start() {
        this.circuitListener.on('NewSideEffect', (data: SideEffectStateManager) => {
            this.addSideEffect(data);
            this.executeSideEffect(data)
        })

        this.circuitRelayer.on("SideEffectConfirmed", (id: string) => {
            console.log("SideEffectConfirmed")
            this.finalize(id);
        })

        this.circuitListener.on('NewHeaderRangeAvailable', (gatewayId: string, blockHeight: number) => {
            this.updateGatewayHeight(gatewayId, blockHeight);
        })
    }
    
    // this function is called by the ExecutionManager
    async executeSideEffect(sideEffectStateManager: SideEffectStateManager) {
        this.instances[sideEffectStateManager.getTarget()].executeTx(sideEffectStateManager)
    }

}

(async () => {
    let exec = new InstanceManager();
    await exec.setup()
    exec.start()
})()
import CircuitListener from "./circuit/listener";
import CircuitRelayer from "./circuit/relayer";
import SubstrateRelayer from "./gateways/substrate/relayer";
import config from "./config.json"
// import { colors } from "./utils/helpers";
import { SideEffect } from "./utils/types";
import { ExecutionManager } from "./utils/executionManager";
import chalk from 'chalk';
// import "dotenv/config"

if (!process.env.SIGNER_KEY) {
  throw Error("missing env var SIGNER_KEY")
}

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


    xtxSfxMap = {}

    async initializeEventListeners() {
        this.circuitListener.on('XTransactionReadyForExec', async (xtxId) => {
            console.log('XTransactionReadyForExec');

            if (this.xtxSfxMap[xtxId]) {
                this.xtxSfxMap[xtxId].forEach( (se) => {
                    this.executionManager.addSideEffect(se)
                } )
            } else {
                console.log('Xtransaction ready for exec ERROR - N side effects stored to xtx id')
            }
             
        })

        this.circuitListener.on('NewSideEffect', async (sideEffect: SideEffect) => {
            console.log('NewSideEffect')
            if (!this.xtxSfxMap[sideEffect.xtxId]) {
                this.xtxSfxMap[sideEffect.xtxId] = [sideEffect]
            } else {
                this.xtxSfxMap[sideEffect.xtxId].push(sideEffect) 
            }
            await this.circuitRelayer.maybeBondInsuranceDeposit(sideEffect)
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
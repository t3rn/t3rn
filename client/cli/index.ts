import config from "./config/setup"
import types from "./config/types.json"
import rpc from "./config/rpc.json"
import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';
import { Sudo } from "./sudo";
import { register } from "./commands/register/register";
import { setOperational } from "./commands/operational";
import {parseTransferArgs} from "./utils/parseArgs";
import {transfer} from "./commands/transfer";

class CircuitCLI {
    circuit: ApiPromise;
    sudo: Sudo;

    async setup() {
        this.circuit = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9944"),
            types: types as any,
            rpc: rpc as any
        })
        this.sudo = new Sudo(this.circuit)
    }

    async close() {
        this.circuit.disconnect()
        process.exit(128);
    }

    async run() {
        await this.setup();
        const args = process.argv[2]
        switch(args) {
            case "register": {
                const data = config.gateways.find(elem => elem.id === process.argv[3])
                if(data) {
                    const tx = await register(this.circuit, data)
                    this.sudo.sudoSignAndSend(tx)
                        .then(() => {
                            console.log("Registered and Activated!")
                            this.close()
                        })
                        .catch(err => {
                            console.log(err)
                            console.log("Registration Failed!")
                            this.close()
                        })
                } else {
                    console.log(`Config for ${process.argv[3]} not found!`)
                    this.close();
                }
                break;
            }
            case "setOperational": {
                const data = config.gateways.find(elem => elem.id === process.argv[3])
                const hasArgument = !!process.argv[4];
                if (data && hasArgument) {
                    const argument = JSON.parse(process.argv[4]);
                    const tx = await setOperational(this.circuit, data, argument)
                    this.sudo.sudoSignAndSend(tx)
                        .then(() => {
                            console.log("setOperational Completed!");
                            this.close();
                        })
                        .catch(err => {
                            console.log(err);
                            console.log("setOperational Failed!");
                            this.close()
                        })
                } else {
                    console.log(`Config or argument for ${process.argv[3]} not found!`)
                    this.close();
                }
                break
            }
            case "transfer": {
                const data = config.gateways.find(elem => elem.id === process.argv[3])
                if(data) {
                    const [amount, sender, receiver, fee] = parseTransferArgs(process.argv, data)
                    transfer(this.circuit, data, amount, sender, receiver, fee)
                        .then(() => {
                            console.log("Transfer Completed!");
                            this.close();
                        })
                        .catch(err => {
                            console.log("Transfer Failed! Error:", err);
                            this.close()
                        })

                } else {
                    console.log(`Config or argument for ${process.argv[3]} not found!`)
                    this.close();
                }
                break
            }
        }
    }
}

(async () => {
    let cli = new CircuitCLI();
    await cli.setup()
    await cli.run()
})()
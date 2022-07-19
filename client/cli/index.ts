import config from "./config/setup"
import types from "./config/types.json"
import rpc from "./config/rpc.json"
import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';
import { Sudo } from "./sudo";
import { register } from "./commands/register/register";
import { setOperational } from "./commands/operational";
import {parseTransferArgs} from "./utils/parseArgs";
import {transfer} from "./commands/transfer";
import * as fs from "fs";

class CircuitCLI {
    circuit: ApiPromise;
    sudo: Sudo;
    signer: any;

    async setup() {
        this.circuit = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9944"),
            types: types as any,
            rpc: rpc as any
        })
        this.sudo = new Sudo(this.circuit)
        const keyring = new Keyring({ type: "sr25519" })
        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri("//Alice")
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)

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
                    const registrationData = await register(this.circuit, data)
                    if (process.argv[4] && process.argv[4] == "--export") {
                        const fileName = './exports/register-' + process.argv[3] + '.json';
                        // @ts-ignore
                        this.exportData(registrationData, fileName)
                    } else {
                        // @ts-ignore
                        this.sudo.sudoSignAndSend(this.circuit.tx.portal.registerGateway(registrationData))
                            .then(() => {
                                console.log("Registered and Activated!")
                                this.close()
                            })
                            .catch(err => {
                                console.log(err)
                                console.log("Registration Failed!")
                                this.close()
                            })
                    }
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

    exportData(data: any, fileName: string) {
        let res = data.toHuman();
        res["encoded"] = data.toHex().substring(2)
        fs.writeFile(fileName, JSON.stringify(res, null, 4), (err) => {
            if(err) {
              console.log(err);
            } else {
              console.log("JSON saved to " + fileName);
              this.close()
            }
        });
    }
}

(async () => {
    let cli = new CircuitCLI();
    await cli.setup()
    await cli.run()
})()
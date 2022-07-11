import config from "./config"
import types from "./types.json"
import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';
import { CircuitRelayer } from "./circuit";
import { register } from "./commands/register/register";
import { setOperational } from "./commands/operational";
import {parseTransferArgs} from "./utils/parseArgs";
import {transfer} from "./commands/transfer";

class CircuitCLI {
    circuit: ApiPromise;
    circuitRelayer: CircuitRelayer;
    signer: any;

    async setup() {
        this.circuit = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9944"),
            types: types as any
        })
        this.circuitRelayer = new CircuitRelayer((this.circuit))
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
                    const tx = await register(this.circuit, data)
                    this.circuitRelayer.sudoSignAndSend(tx)
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
                    this.circuitRelayer.sudoSignAndSend(tx)
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
                    console.log([amount, sender, receiver, fee])
                    const tx = transfer(this.circuit, data, amount, sender, receiver, fee);
                     this.circuitRelayer.signAndSend(tx)
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
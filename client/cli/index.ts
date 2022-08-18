import config from "./config/setup"
import types from "./config/types.json"
import rpc from "./config/rpc.json"
import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';
import { CircuitRelayer } from "./circuitRelayer";
import { register } from "./commands/register/register";
import { setOperational } from "./commands/operational";
import {parseRegisterArgs, parseSubmitHeaderArgs, parseTransferArgs} from "./utils/parseArgs";
import {onExtrinsicTrigger} from "./commands/onExtrinsicTrigger";
import * as fs from "fs";
import {submitHeader} from "./commands/submit_header/submit_header";
import { amountLeArr, encodeExport, optionalInsurance } from "./utils/encoder";

import { Command } from 'commander';
import {addressStringToPubKey} from "./utils/decoder";
const program = new Command();

program
  .name('t3rn CLI')
  .description('CLI for interacting with the t3rn circuit')
  .version('0.1.0');

class CircuitCLI {
    circuit: ApiPromise;
    circuitRelayer: CircuitRelayer;
    signer: any;

    async setup() {
        this.circuit = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9944"),
            types: types as any,
            rpc: rpc as any
        })
        const keyring = new Keyring({ type: "sr25519" })
        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri("//Alice")
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
        this.circuitRelayer = new CircuitRelayer(this.circuit, this.signer)
        console.log(this.signer.address)

    }

    async close() {
        this.circuit.disconnect()
        process.exit();
    }

    async error() {
        this.circuit.disconnect()
        process.exit(1);
    }

    async register(id: string, teleport: number, exportArgs: boolean, exportName: string) {
        let data: any = config.gateways.find(elem => elem.id === id)
        if(data) {
            if(data.registrationData?.parachain !== null) {
                // @ts-ignore
                data.relaychainRpc = config.gateways.find(elem => elem.id === data.registrationData.parachain.relayChainId).rpc
            }
            const registrationData: any = await register(this.circuit, data, teleport)

            const tx = this.circuit.tx.portal.registerGateway(
                registrationData[0].url,
                registrationData[0].gateway_id,
                registrationData[0].gateway_abi,
                registrationData[0].gateway_vendor,
                registrationData[0].gateway_type,
                registrationData[0].gateway_genesis,
                registrationData[0].gateway_sys_props,
                registrationData[0].allowed_side_effects,
                registrationData[0].registration_data.toHex()
            );
            let submissionHeight = await this.circuitRelayer.sudoSignAndSend(tx)
                .catch(err => {
                    console.log(err)
                    console.log("Registration Failed!")
                    this.error()
                })

            if (exportArgs) {
                const fileName = `./exports/` + exportName + '.json';
                // @ts-ignore
                await this.exportData(registrationData, fileName, "register", submissionHeight)
            } else {
                this.close()
            }

        } else {
            console.log(`Config for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async setOperational(id: string, operational: boolean, exportArgs: boolean, exportName: string) {
        const data = config.gateways.find(elem => elem.id === id)
        if (data) {
            const transactionArgs= await setOperational(this.circuit, data, operational)
            const submissionHeight = await this.circuitRelayer.sudoSignAndSend(this.circuit.tx.portal.setOperational(transactionArgs?.gatewayId, transactionArgs?.operational))
                .catch(err => {
                    console.log(err);
                    console.log("setOperational Failed!");
                    this.error()
                })

            if (exportArgs) {
                const fileName = `./exports/` + exportName + '.json';
                // @ts-ignore
                await this.exportData([transactionArgs], fileName, "set-operational", submissionHeight)
            } else {
                this.close()
            }

        } else {
            console.log(`Config or argument for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async submitHeaders(id: string, exportArgs: boolean, exportName: string) {
        const gatewayData = config.gateways.find(elem => elem.id === id)
        if(gatewayData) {
            if(gatewayData.registrationData?.parachain !== null) {
                // @ts-ignore
                gatewayData.relaychainRpc = config.gateways.find(elem => elem.id === gatewayData.registrationData.parachain.relayChainId).rpc
            }
            const transactionArgs: any[] = await submitHeader(this.circuit, gatewayData, id)
            const submissionHeight = await this.circuitRelayer.submitHeaders(transactionArgs)
                .catch(err => {
                    console.log(err)
                    console.log("Header Submission Failed!")
                    this.error()
                })

            if (exportArgs) {
                const fileName = `./exports/` + exportName + '.json';
                // @ts-ignore
                await this.exportData(transactionArgs, fileName, "submit-headers", submissionHeight)
            } else {
                this.close()
            }

        } else {
            console.log(`Config for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async transfer(data: any, sequential: boolean) {
        const gatewayData = config.gateways.find(elem => elem.id === data.target)

        if(gatewayData) {
            if(data.receiver === '') data.receiver = gatewayData.transferData.receiver;
            const transactionArgs: any = onExtrinsicTrigger(this.circuit, [data], sequential, this.signer.address)
            // @ts-ignore
            let submissionNumber: number = await this.circuitRelayer.onExtrinsicTrigger(transactionArgs)
                .catch(err => {
                    console.log("Transfer Failed! Error:", err);
                    this.error()
                })

            if (data.exportArgs) {
                const fileName = `./exports/` + data.exportName + '.json';
                this.exportData([transactionArgs], fileName, "transfer", submissionNumber)
            } else {
                this.close()
            }

        } else {
            console.log(`Config or argument for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async submitSideEffects(path: string, exportArgs: boolean, exportName: string) {
        if (!fs.existsSync(path)) {
             console.log("File doesn't exist!")
             this.error()
         }

        const data = (await import('./' + path)).default;

        // Check we have an config for each SideEffect
        data.sideEffects.forEach(effect => {
            if (!config.gateways.find(entry => entry.id === effect.target)) {
                console.log(`Gateway for SideEffect ${effect.type} not found!`)
                this.error()
            }
        })

        const transactionArgs: any = onExtrinsicTrigger(this.circuit, data.sideEffects, data.sequential, this.signer.address)
        // @ts-ignore
        let submissionNumber: number = await this.circuitRelayer.onExtrinsicTrigger(transactionArgs)
            .catch(err => {
                console.log("Transfer Failed! Error:", err);
                this.error()
            })

        if (exportArgs) {
            const fileName = `./exports/` + exportName + '.json';
            this.exportData([transactionArgs], fileName, "transfer", submissionNumber)
        } else {
            this.close()
        }
    }

    exportData(data: any, fileName: string, transactionType: string, submissionHeight: number) {
        let deepCopy;
        // since its pass-by-reference
        if(Array.isArray(data)) {
            deepCopy = [...data];
        } else {
            deepCopy = {...data};
        }
        let encoded = encodeExport(deepCopy, transactionType, submissionHeight);
        fs.writeFile(fileName, JSON.stringify(encoded, null, 4), (err) => {
            if(err) {
              console.log(err);
              this.error();
            } else {
              console.log("JSON saved to " + fileName);
              this.close();
            }
        });
    }
}

program.command('register')
      .description('Register a gateway on the t3rn blockchain')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .option('-t, --teleport <number>', 'how many epochs the registration should go back.', "0")
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          cli.register(id, parseInt(options.teleport), options.export, options.output)
      });

program.command('set-operational')
      .description('Activate/deactivate a gateway')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .argument('operational <bool>', 'gateway_id as specified in setup.ts')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, operational, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          operational = operational === "true" ? true : false;
          cli.setOperational(id, operational, options.export, options.output)
      });

program.command('submit-headers')
      .description('Submit the latest headers of a gateway to portal. All available finalized headers will be added.')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          cli.submitHeaders(id, options.export, options.output)
      });

program.command('transfer')
      .description('Triggers a transfer SideEffect, sending the targets nativ asset')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .option('-a --amount <float>', 'transfer amount', '1')
      .option('-r --receiver <string>', 'receiver address', '')
      .option('-b --bond <float>', 'The bond required for execution', '0')
      .option('--reward <float>', 'The reward payed out (not sure for what)', '0')
      .option('--executioner <string>', 'enforce executioner address')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          options.target = id
          options.type = "tran"
          cli.transfer(options, false)
      });

program.command('submit-side-effects')
      .description('Submits SideEffects based on input file')
      .argument('path <string>', 'path to file')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (path, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          console.log(options.export)
          cli.submitSideEffects(path, options.export, options.output)
      });

program.parse();
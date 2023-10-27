const assert = require("assert")
const path = require('path')
import {ApiPromise, WsProvider, Keyring} from "@t3rn/sdk"

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'
const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'

async function checkEvent(api) {
    return new Promise(resolve => {
        const eventSection = 'xdns';
        const eventName = 'GatewayRecordStored';

        api.query.system.events((events) => {
            const expectedEvents = events
                .toHuman()
                .filter((event) => event.event.section === eventSection && event.event.method === eventName && event.event.data[0] === 'roco');

            if (expectedEvents.length > 0) {
                console.log(`✅ Event ${eventSection}.${eventName} emitted for ${expectedEvents[0].event.data}`);
                resolve(1)
            } else {
                console.log(`⏳ Event ${eventSection}.${eventName} for roco not yet emitted.`);
            }
        });
    });
}

async function registerAsset(api, owner, assetMetadata)



async function run(nodeName, networkInfo, args) {
    const { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName]
    const api = await zombie.connect(wsUri, userDefinedTypes)

    try {

        // Register ROc on t0rn
        const keyring = new Keyring({ type: "sr25519" })
        const signer = keyring.addFromUri("//Alice")
        //await api.tx.assets.createAsset()
    } catch (err) {
        console.error(err);
        process.exit(1)
    }

    return 1
    //const result = await checkEvent(api);
    //return result;

    /*
       const command = `pnpm cli xcmTransfer
        --signer "//Bob"
        --type "relay"
        --endpoint "ws://127.0.0.1:9933"
        --dest "3333"
        --recipient "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
        --target-asset "ROC"
        --target-amount 2000000000000'
       `
       const cliPath = path.join(__dirname, '../../../client/packages/cli')

       try {
           exec(command, { cwd: cliPath }, (error) => {
             if (error) {
               console.error(`Error executing command "${command}": ${error}`);
               process.exit(1)
             }
           });
       } catch (err) {
           console.error(err);
           process.exit(1)
       }

       const result = await checkEvent(api);
       return result;
   }
   */
}

module.exports = { run }
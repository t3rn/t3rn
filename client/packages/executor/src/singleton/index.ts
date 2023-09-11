import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { Gateway } from "../../config/config";
import { logger } from "../logging";

export class SingletonComponent {
    // maps gateways to there nonce values
  
    nonces: {
        [id: string]: number
    }
 

    async setup(config: Gateway[]) {
        for (let i = 0; i < config.length; i++) {
            const client = await ApiPromise.create({
                provider: new WsProvider(config[i].rpc),
            });
            const keyring = new Keyring({ type: "sr25519" });
            const signer = config[i].signerKey
                ? keyring.addFromMnemonic(config[i].signerKey)
                : keyring.addFromUri("//Executor//default");
            this.nonces[config[i].id] = await this.fetchNonce(client, signer.address);
        }
        logger.info("Singleton setup complete");
    }

    async fetchNonce(api: ApiPromise, address: string): Promise<number>{
        return await api.rpc.system.accountNextIndex(address).then((nextIndex) => {
            // @ts-ignore - property does not exist on type
            return parseInt(nextIndex.toNumber());
        });

    }

    setNonce(id: string, value: number): void{
        this.nonces[id] = value;
    }

    getNonce(id: string): number {
        return this.nonces[id]
    }
}
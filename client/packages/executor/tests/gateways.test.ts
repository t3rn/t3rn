import { default as chai, expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { SubstrateRelayer } from "../src/gateways/substrate/relayer";
import { mock } from "ts-mockito";
// import { Batch } from "../src/attestationManager/batch";
import { Sdk } from "@t3rn/sdk";
import { Prometheus } from "../src/prometheus";
import { config } from "../config/config"

chai.use(chaiAsPromised);
chai.should();

describe("Gateways", () => {
    let relayer: SubstrateRelayer;
    before(async () => {
        const prometheus = new Prometheus();
        const gateways = config.gateways;
        const gateway = gateways[0]
        relayer = new SubstrateRelayer()
        await relayer.setup(gateway, prometheus);
    })

    
})
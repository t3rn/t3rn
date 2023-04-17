import { default as chai, expect } from "chai"
import chaiAsPromised from "chai-as-promised"
import { jestSnapshotPlugin } from "mocha-chai-jest-snapshot"
import { mkdir } from "fs/promises"
import { Execution } from "../src/executionManager/execution"
import { SideEffect } from "../src/executionManager/sideEffect"

chai.use(chaiAsPromised)
chai.use(jestSnapshotPlugin())
chai.should()

describe("Serialization", () => {
    let logger = { logsDir: "~/.t3rn-executor-alina/logs" }
    let sdk = { signer: { address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" }, gateways: {ROCO:{id:"ROCO",createSfx:{tran(x) {return x}}}}, circuit:{toFloat(x){return x}} }
    let misc = {
        executorName: "alina",
        logsDir: logger.logsDir,
        circuitRpc: "wss://ws.t0rn.io",
        circuitSignerAddress: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        circuitSignerSecret: "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a",
        gatewayId: "ROCO"
    }

    describe("Execution", () => {
        let xtx

        beforeEach(async () => {
            await mkdir(logger.logsDir, { recursive: true })
            xtx = new Execution(
                [
                    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    {
                        toHex() {
                            return "0xacabacab"
                        },
                    },
                    [],
                    [],
                ],
                sdk as any,
                null!,
                null!,
                logger,
                misc
            )
        })

        it("should rountrip xtx", async () => {
            let input = xtx.toJSON()
            expect(input).toMatchSnapshot()

            let restored = Execution.fromJSON(JSON.parse(JSON.stringify(input)))
            let output = restored.toJSON()

            expect(output).to.deep.equal(input)
            expect(restored).to.be.instanceOf(Execution)
        })
    })

    describe("SideEffect", () => {
        let sfx

        beforeEach(async () => {
            await mkdir(logger.logsDir, { recursive: true })
            sfx = new SideEffect(
                {
                    action: { toHuman() {return "tran"}},
                    encodedArgs: [],
                    target: { toU8a() {return Uint8Array.from(Buffer.from("ROCO"))}},
                    from: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                    value: 50,
                    maxReward: 1,
                    insurance: 100,
                },
                "0xacabacab",
                "0xacabacabacabacab",
                sdk as any,
                null!,
                null!,
                logger,
                misc
            )
        })

        it("should rountrip sfx", async () => {
            let input = sfx.toJSON()
            expect(input).toMatchSnapshot()

            let restored = SideEffect.fromJSON(JSON.parse(JSON.stringify(input)))
            let output = restored.toJSON()

            expect(output).to.deep.equal(input)
            expect(restored).to.be.instanceOf(SideEffect)
        })
    })
})

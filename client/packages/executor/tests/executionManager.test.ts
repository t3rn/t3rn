import { default as chai, expect } from "chai"
import chaiAsPromised from "chai-as-promised"
import { jestSnapshotPlugin } from "mocha-chai-jest-snapshot"
import { mkdir } from "fs/promises"
import { Execution } from "../src/executionManager/execution"
import { SideEffect } from "src/executionManager/sideEffect"

chai.use(chaiAsPromised)
chai.use(jestSnapshotPlugin())
chai.should()

describe("Serialization", () => {
    const logger = { logsDir: "~/.t3rn-executor-alina/logs" }
    const sdk = { signer: { address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" } }
    const misc = {
        executorName: "alina",
        logsDir: logger.logsDir,
        circuitRpc: "ws://localhost:9944",
        circuitSignerAddress: sdk.signer.address,
        circuitSignerSecret: `0x${"acab".repeat(16)}`,
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
                [
                    {
                        from: "0xdDf4C5025D1A5742cF12F74eEC246d4432c295e4",
                        to: "0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990",
                        value: 50,
                        maxReward: 1,
                        insurance: 100,
                    },
                ],
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

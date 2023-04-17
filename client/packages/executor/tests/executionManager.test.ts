import { default as chai, expect } from "chai"
import chaiAsPromised from "chai-as-promised"
import { jestSnapshotPlugin } from "mocha-chai-jest-snapshot"
import { mkdir } from "fs/promises"
import { Execution } from "../src/executionManager/execution"

chai.use(chaiAsPromised)
chai.use(jestSnapshotPlugin())
chai.should()

describe("Execution", () => {
    describe("Serialization", () => {
        let xtx

        beforeEach(async () => {
            const logger = { logsDir: "~/.t3rn-executor-alina/logs" }
            const sdk = { signer: { address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" } }
            const misc = {
                executorName: "alina",
                logsDir: "~/.t3rn-executor-alina/logs",
                circuitRpc: "ws://localhost:9944",
                circuitSignerAddress: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                circuitSignerSecret: `0x${"acab".repeat(16)}`,
            }
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

        it("should rountrip", async () => {
            let input = xtx.toJSON()
            expect(before).toMatchSnapshot()

            let restored = Execution.fromJSON(JSON.parse(JSON.stringify(xtx.toJSON())))
            let output = restored.toJSON()

            expect(output).to.deep.equal(input)
            expect(restored).to.be.instanceOf(Execution)
        })
    })
})

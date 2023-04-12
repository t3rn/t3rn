import { default as chai, expect } from "chai"
import chaiAsPromised from "chai-as-promised"
import { join } from "path"
import { homedir } from "os"
import { Instance } from "../src"

chai.use(chaiAsPromised)
chai.should()

describe("Instance", () => {
    describe("Configuration", () => {
        let name = "alina"

        it("should throw if signer keys are missing", async () => {
            let instance = new Instance()

            instance.loadConfig(name).should.be.rejectedWith(Error, "Instance::loadConfig: missing circuit signer key")
        })

        it("should throw if providing malformatted signer keys", async () => {
            process.env.CIRCUIT_SIGNER_KEY = `0x${"hi".repeat(16)}`
            process.env.ROCO_GATEWAY_SIGNER_KEY = `0x${"hi".repeat(16)}`
            let instance = new Instance()

            instance.loadConfig(name).should.be.rejectedWith(Error, "Instance::loadConfig: missing circuit signer key")
        })

        it("should load custom config", async () => {
            // set bogus signer keys bc they are required
            process.env.CIRCUIT_SIGNER_KEY = `0x${"dead".repeat(16)}`
            process.env.ROCO_GATEWAY_SIGNER_KEY = `0x${"dead".repeat(16)}`
            let instance = new Instance()

            expect(instance.config).to.be.undefined

            let config = await instance.loadConfig(name)

            expect(instance.config).to.not.be.undefined
            expect(config).to.equal(instance.config)
        })

        it("should persist custom config", async () => {
            //TODO
            //join(homedir(), `.t3rn-executor-${name}`, "config.json")
        })
    })
})

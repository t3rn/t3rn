import { default as chai, expect } from "chai"
import chaiAsPromised from "chai-as-promised"
import { join } from "path"
import { homedir } from "os"
import { existsSync } from "fs"
import { readFile, readdir, mkdir, unlink } from "fs/promises"
import { Instance } from "../src"

chai.use(chaiAsPromised)
chai.should()

describe("Instance", () => {
    let name = "alina"

    describe("Configuration", () => {
        let conf = join(homedir(), `.t3rn-executor-${name}`, "config.json")
        let instance

        beforeEach(async () => {
            if (existsSync(conf)) {
                await unlink(conf)
            }
            process.env.CIRCUIT_SIGNER_KEY = `0x${"dead".repeat(16)}`
            process.env.ROCO_GATEWAY_SIGNER_KEY = `0x${"dead".repeat(16)}`
            instance = new Instance(name, false /*logToDisk*/)
            instance.logger = { warn() {}, info() {} }
        })

        it("should throw if signer keys are missing", async () => {
            process.env.CIRCUIT_SIGNER_KEY = undefined
            process.env.ROCO_GATEWAY_SIGNER_KEY = undefined

            instance.loadConfig().should.be.rejectedWith(Error, "Instance::loadConfig: missing circuit signer key")
        })

        it("should load custom config", async () => {
            expect(instance.config).to.be.undefined

            let config = await instance.loadConfig()

            expect(instance.config).to.not.be.undefined
            expect(config).to.deep.equal(instance.config)
        })

        it("should persist custom config", async () => {
            expect(existsSync(conf)).to.be.false

            let config = await instance.loadConfig()

            let stored = await readFile(conf, "utf8").then((str) => JSON.parse(str))
            expect(existsSync(conf)).to.be.true
            expect(stored).to.deep.equal(config)
        })
    })

    describe("Logs", () => {
        let logs = join(homedir(), `.t3rn-executor-${name}`, "logs")
        let instance

        beforeEach(async () => {
            await mkdir(logs, { recursive: true })
            await readdir(logs).then((logFiles) => Promise.all(logFiles.map((logFile) => unlink(join(logs, logFile)))))
            process.env.CIRCUIT_SIGNER_KEY = `0x${"dead".repeat(16)}`
            process.env.ROCO_GATEWAY_SIGNER_KEY = `0x${"dead".repeat(16)}`
            instance = new Instance()
            instance.logger = { warn() {}, info() {} }
        })

        it("should conditionally log to disk", async () => {
            let logToDisk = true
            let logFiles = await readdir(logs)
            expect(logFiles.length).to.equal(0)

            await instance.configureLogging(name, logToDisk)
            await instance.logger.info("hallo")

            logFiles = await readdir(logs)
            expect(logFiles.length).to.equal(1)
            let logged = await readFile(join(logs, logFiles[0]), "utf8")
            expect(logged.length).to.be.greaterThan(0)
            expect(logged).to.match(/hallo/)
        })
    })
})

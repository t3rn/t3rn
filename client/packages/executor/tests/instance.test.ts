import { default as chai, expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { jestSnapshotPlugin } from "mocha-chai-jest-snapshot";
import { join } from "path";
import { homedir } from "os";
import { existsSync } from "fs";
import { readFile, readdir, mkdir, unlink } from "fs/promises";
import { Executor } from "../src";
import { mock } from "ts-mockito";

chai.use(chaiAsPromised);
chai.use(jestSnapshotPlugin());
chai.should();

describe("Executor", () => {
  const name = "alina";

  describe("Configuration", () => {
    const conf = join(homedir(), `.t3rn-executor-${name}`, "config.json");
    let instance;

    beforeEach(async () => {
      if (existsSync(conf)) {
        await unlink(conf);
      }
      process.env.CIRCUIT_SIGNER_KEY = `0x${"acab".repeat(16)}`;
      process.env.ROCO_GATEWAY_SIGNER_KEY = `0x${"acab".repeat(16)}`;
      instance = new Executor(name, false, mock());
      instance.logger = { warn() {}, info() {} };
    });

    it("should throw if signer keys are missing", async () => {
      process.env.CIRCUIT_SIGNER_KEY = undefined;
      process.env.ROCO_GATEWAY_SIGNER_KEY = undefined;

      instance
        .loadConfig()
        .should.be.rejectedWith(
          Error,
          "Executor::loadConfig: missing circuit signer key",
        );
    });

    it("should throw if signer keys are malformatted", async () => {
      process.env.CIRCUIT_SIGNER_KEY = "acab";
      process.env.ROCO_GATEWAY_SIGNER_KEY = "acab";

      instance
        .loadConfig()
        .should.be.rejectedWith(
          Error,
          "Executor::loadConfig: missing circuit signer key",
        );

      // reset to bogus substrate private key for the remainder
      // somehow the beforeEach hook resets don't suffice
      process.env.CIRCUIT_SIGNER_KEY = `0x${"acab".repeat(16)}`;
      process.env.ROCO_GATEWAY_SIGNER_KEY = `0x${"acab".repeat(16)}`;
    });

    it("should load custom config", async () => {
      expect(instance.config).to.be.undefined;

      const config = await instance.loadConfig();

      expect(instance.config).to.not.be.undefined;
      expect(config).to.deep.equal(instance.config);
      expect(instance.config).toMatchSnapshot();
    });

    it("should persist custom config", async () => {
      expect(existsSync(conf)).to.be.false;

      const config = await instance.loadConfig();

      expect(existsSync(conf)).to.be.true;
      const stored = await readFile(conf, "utf8").then(JSON.parse);
      expect(stored).to.deep.equal(config);
      expect(stored).toMatchSnapshot();
    });
  });

  describe("Logs", () => {
    const logs = join(homedir(), `.t3rn-executor-${name}`, "logs");
    let instance;

    beforeEach(async () => {
      await mkdir(logs, { recursive: true });
      await readdir(logs).then((logFiles) =>
        Promise.all(logFiles.map((logFile) => unlink(join(logs, logFile)))),
      );
      process.env.CIRCUIT_SIGNER_KEY = `0x${"acab".repeat(16)}`;
      process.env.ROCO_GATEWAY_SIGNER_KEY = `0x${"acab".repeat(16)}`;
      instance = new Executor(name, false, mock());
    });

    it("should not log to disk", async () => {
      let logFiles = await readdir(logs);
      expect(logFiles.length).to.equal(0);

      await instance.configureLogging();
      instance.logger = { warn() {}, info() {} } as any;
      await instance.logger.info("hallo");

      logFiles = await readdir(logs);
      expect(logFiles.length).to.equal(0);
    });

    it("should log to disk", async () => {
      const instance = new Executor(name, true, mock());
      let logFiles = await readdir(logs);
      expect(logFiles.length).to.equal(0);

      await instance.configureLogging();
      await instance.logger.info("hallo");

      logFiles = await readdir(logs);
      expect(logFiles.length).to.equal(1);
      const logged = await readFile(join(logs, logFiles[0]), "utf8");
      expect(logged).to.match(/hallo/);
    });
  });
}).afterAll(() => {
  process.exit(0);
});

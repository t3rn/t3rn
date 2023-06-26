import { EVMRelayer } from "../src/gateways/evm/relayer";
import { expect } from "chai";

// import { Sdk } from "@t3rn/sdk";
// import { mock } from "ts-mockito";

describe("basic evm relayer setup", () => {
  it("should be a class", () => {
    expect(EVMRelayer).to.be.a("function");
  });

  it("should have a constructor", () => {
    expect(EVMRelayer).to.have.property("constructor");
  });

  it("basic creation of an EVM relayer instance", () => {
    const evmRelayer = new EVMRelayer();

    expect(evmRelayer).to.be.instanceof(EVMRelayer);
  });
});

describe("basic evm relayer functionality", () => {
  const evmRelayer = new EVMRelayer();
  evmRelayer.setup("0x", "0x")
})

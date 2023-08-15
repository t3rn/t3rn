#!/bin/env node
import fs from "fs"
import { Command } from "commander"
import { handleInitCmd, initConfigFile } from "./commands/init.ts"
import { CONFIG_FILE } from "@/consts.ts"
import { wrapCryptoWaitReady } from "./utils/fns.ts"
import { handleRegisterCmd } from "./commands/register/register.ts"
import { handleSubmitCmd } from "./commands/submit/submit.ts"
import { handleBidCmd } from "./commands/bid.ts"
import { handleDgfCmd } from "./commands/dgf.ts"
import { handleEstimateGasFee } from "./commands/estimate/gas.ts"
import { handleEstimateMaxReward } from "./commands/estimate/reward.ts"
import { handleEstimateBidAmount } from "./commands/estimate/bid.ts"

if (!fs.existsSync(CONFIG_FILE)) {
  initConfigFile(CONFIG_FILE)
}
const withExportMode = (program: Command) =>
  program.option("-x, --export", "Export extrinsic data to a file")

const program = new Command()

program
  .name("t3rn CLI")
  .description("CLI for interacting with the t3rn circuit")
  .version("0.1.0")

program
  .command("init")
  .option("-c, --config [file-path]", "Generate a config template")
  .option("-t, --transfer [file-path]", "Generate a transfer template")
  .description("Generate a config or transfer template")
  .action(handleInitCmd)

withExportMode(
  program
    .command("register")
    .option("-g, --gateway <id>", "ID of the gateway to register")
    .description("Register a gateway with the t3rn circuit")
    .action(wrapCryptoWaitReady(handleRegisterCmd)),
)

withExportMode(
  program
    .command("submit")
    .option("-s, --sfx <file-path>", "Path to the sfx JSON file")
    .option(
      "-h, --headers <gateway_id>",
      "Submit the latest headers of a gateway to portal. All available finalized headers will be added.",
    )
    .description("Submit an extrinic to the t3rn circuit")
    .action(wrapCryptoWaitReady(handleSubmitCmd)),
)

withExportMode(
  program
    .command("bid")
    .description("Bid on an execution as an Executor")
    .argument("sfxId <string>", "sfxId of the side effect to bid on")
    .argument("amount <float>", "bid amount")
    .action(wrapCryptoWaitReady(handleBidCmd)),
)

withExportMode(
  program
    .command("dgf")
    .description(
      "Generate side effects data with specific error modes for testing purposes on the chain.",
    )
    .option(
      "-s, --sfx <file-path>",
      "Path to the sfx JSON file",
      "transfer.json",
    )
    .option(
      "-t, --timeout <timeout>",
      "Timeout in seconds for waiting for events from the chain",
      "30",
    )
    .action(wrapCryptoWaitReady(handleDgfCmd)),
)

withExportMode(
  program
    .command("estimate-gas-fee")
    .description("Estimate the gas fee required for an execution")
    .requiredOption("-t, --target <name>", "The target name")
    .requiredOption("-a, --action <action>", "The execution action")
    .option(
      "-o, --args <action>",
      "The execution arguments. It's value can be a speed mode, a EVM call estimation or a side-effect JSON string",
    )
    .option("-s, --sfx <action>", "The SFX file path")
    .action(wrapCryptoWaitReady(handleEstimateGasFee)),
)

withExportMode(
  program
    .command("estimate-max-reward")
    .description("Estimate the max reward for an execution")
    .requiredOption("-a, --action <action>", "The execution action")
    .requiredOption("-b, --base-asset <symbol>", "The base asset")
    .requiredOption("-t, --target <name>", "The target name")
    .requiredOption("--target-asset <symbol>", "The target asset")
    .requiredOption(
      "--target-amount <amount>",
      "The amount of the target asset",
    )
    .requiredOption(
      "--over-spend <percent>",
      "The percentage of the target amount to be used as a profit margin",
    )
    .option(
      "-o, --args <action>",
      "The execution arguments. It's value can be a speed mode, a EVM call estimation or a side-effect JSON string",
    )
    .option("-s, --sfx <action>", "The SFX file path")
    .action(wrapCryptoWaitReady(handleEstimateMaxReward)),
)

withExportMode(
  program
    .command("estimate-bid-amount")
    .description(
      "Estimate the bid amount with a specified profit margin for an execution",
    )
    .requiredOption("-t, --target <name>", "The target name")
    .requiredOption("-a, --action <action>", "The execution action")
    .requiredOption("-p, --profit-margin <profit-margin>", "The profit margin")
    .option(
      "-o, --args <action>",
      "The execution arguments. It's value can be a speed mode, a EVM call estimation or a side-effect JSON string",
    )
    .option("-s, --sfx <action>", "The SFX file path")
    .action(wrapCryptoWaitReady(handleEstimateBidAmount)),
)

program.parse(process.argv)

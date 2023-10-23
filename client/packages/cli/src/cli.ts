#!/bin/env node
import { Command } from "commander"
import { handleInitCmd } from "./commands/init.ts"
import { wrapCryptoWaitReady } from "./utils/fns.ts"
import { handleRegisterCmd } from "./commands/registerGateway/register.ts"
import { handleRebootCommand } from "./commands/rebootGateway/index.ts"
import { handleSubmitCmd } from "./commands/submit/submit.ts"
import { handleBidCmd } from "./commands/bid.ts"
import { handleDgfCmd } from "./commands/dgf.ts"
import { handleEstimateMaxReward } from "./commands/estimate.ts"
import { handlePurgeGatewayCommand } from "./commands/purgeGateway/index.ts"
import { handlePurgeTokenCommand } from "./commands/purgeToken/index.ts"
import { handleXcmTransferCommand } from "./commands/xcmTransfer/index.ts"
import { handleResetGatewayCommand } from "./commands/resetGateway/index.ts"

const withExportMode = (program: Command) =>
  program.option("-x, --export", "Export extrinsic data to a file")

const program = new Command()

program
  .name("t3rn CLI")
  .description("CLI for interacting with the t3rn blockchain")
  .version("0.1.0")

program
  .command("init")
  .description("Generate a config or transfer template")
  .option("-t, --transfer [file-path]", "Generate a transfer template")
  //.option("-x, --xtransfer [file-path]", "Generate a cross-chain transfer template") // enable when agreed on XCM workflow
  .action(handleInitCmd)

withExportMode(
  program
    .command("registerGateway")
    .argument("gateway", "ID of the gateway to register")
    .description("Register a gateway with the circuit")
    .action(wrapCryptoWaitReady(handleRegisterCmd)),
)

withExportMode(
  program
    .command("rebootGateway")
    .argument("vendor")
    .description("Reboot a gateway")
    .action(wrapCryptoWaitReady(handleRebootCommand)),
)

program
  .command("resetGateway")
  .argument("gateway")
  .description("Reset gateway")
  .option("-f, --force", "Force on live chain")
  .action(wrapCryptoWaitReady(handleResetGatewayCommand)),
  withExportMode(
    program
      .command("purgeGateway")
      .argument("gateway")
      .description("Purge a gateway")
      .option("-f, --force", "Force on live chain")
      .action(wrapCryptoWaitReady(handlePurgeGatewayCommand)),
  )

withExportMode(
  program
    .command("purgeToken")
    .argument("token")
    .description("Purge a token")
    .option("-f, --force", "Force on live chain")
    .action(wrapCryptoWaitReady(handlePurgeTokenCommand)),
)

withExportMode(
  program
    .command("submit")
    .option("-s, --sfx <file-path>", "Path to the sfx JSON file")
    .option(
      "-h, --headers <gateway_id>",
      "Submit the latest headers of a gateway to portal. All available finalized headers will be added.",
    )
    .description("Submit an extrinic to the t3rn blockchain")
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

program
  .command("estimate")
  .requiredOption("--action <action>", "The execution action")
  .requiredOption("--base-asset <symbol>", "The base asset")
  .requiredOption("--target <name>", "The target name")
  .requiredOption("--target-asset <symbol>", "The target asset")
  .requiredOption("--target-amount <amount>", "The amount of the target asset")
  .requiredOption(
    "--over-spend <percent>",
    "The percentage of the target amount to be used as a profit margin",
  )
  .description("Estimate the max reward for an execution")
  .action(handleEstimateMaxReward),
  withExportMode(
    program
      .command("xcmTransfer")
      .description("Cross-chain transfer of assets using XCM")
      .requiredOption("--signer <string>", "The signer of the transaction")
      .requiredOption("--type <string>", "The type of XCM transfer")
      .requiredOption(
        "--endpoint <string>",
        "The RPC endpoint from which the XCM transaction will be submitted",
      )
      .requiredOption("--dest <string>", "The destination chain")
      .requiredOption("--recipient <string>", "The recipient address")
      .requiredOption("--target-asset <symbol>", "The target asset")
      .requiredOption(
        "--target-amount <amount>",
        "The amount of the target asset",
      )
      .action(handleXcmTransferCommand),
  )

program.parse(process.argv)

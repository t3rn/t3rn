#!/bin/env node
import { Command } from "commander"
import { handleInitCmd } from "./commands/init.ts"
import { wrapCryptoWaitReady } from "./utils/fns.ts"
import { handleRegisterCmd } from "./commands/register/register.ts"
import { handleSubmitCmd } from "./commands/submit/submit.ts"
import { handleBidCmd } from "./commands/bid.ts"
import { handleDgfCmd } from "./commands/dgf.ts"

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
    .action(wrapCryptoWaitReady(handleRegisterCmd))
)

withExportMode(
  program
    .command("submit")
    .option("-s, --sfx <file-path>", "Path to the sfx JSON file")
    .option(
      "-h, --headers <gateway_id>",
      "Submit the latest headers of a gateway to portal. All available finalized headers will be added."
    )
    .description("Submit an extrinic to the t3rn circuit")
    .action(wrapCryptoWaitReady(handleSubmitCmd))
)

withExportMode(
  program
    .command("bid")
    .description("Bid on an execution as an Executor")
    .argument("sfxId <string>", "sfxId of the side effect to bid on")
    .argument("amount <float>", "bid amount")
    .action(wrapCryptoWaitReady(handleBidCmd))
)

withExportMode(
  program
    .command("dgf")
    .description("Generate data for unhpappy paths")
    .argument("sfxFile <string>", "folder from where the sfx is loaded")
    .action(wrapCryptoWaitReady(handleDgfCmd))
)

program.parse(process.argv)

#!/bin/env node
import { Command } from "commander"
import { handleInitCmd } from "./commands/init.ts"
import { wrapCryptoWaitReady } from "./utils/fns.ts"
import { handleRegisterCmd } from "./commands/register/register.ts"
import { handleSubmitCmd } from "./commands/submit/submit.ts"
import { handleSetOperational } from "./commands/set_operational.ts"
import { handleBidCmd } from "./commands/bid.ts"

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

program
  .command("register")
  .option("-g, --gateway <id>", "ID of the gateway to register")
  .description("Register a gateway with the t3rn circuit")
  .action(wrapCryptoWaitReady(handleRegisterCmd))

program
  .command("submit")
  .option("-e, --extrinsic <file-path>", "Path to the extrinsc JSON file")
  .option(
    "-h, --headers <gateway_id>",
    "Submit the latest headers of a gateway to portal. All available finalized headers will be added."
  )
  .description("Submit an extrinic to the t3rn circuit")
  .action(wrapCryptoWaitReady(handleSubmitCmd))

program
  .command("set-operational")
  .argument("gateway_id <string>", "gateway_id as specified in setup.ts")
  .argument("operational <bool>", "gateway_id as specified in setup.ts")
  .description("Set a gateway operational")
  .action(wrapCryptoWaitReady(handleSetOperational))

program
  .command("bid")
  .description("Bid on an execution as an Executor")
  .argument("sfxId <string>", "sfxId of the side effect to bid on")
  .argument("amount <float>", "bid amount")
  .action(wrapCryptoWaitReady(handleBidCmd))

program.parse(process.argv)

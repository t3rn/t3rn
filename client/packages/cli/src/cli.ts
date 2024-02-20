#!/bin/env node
import { Command } from 'commander'
import { handleInitCmd } from './commands/init.ts'
import { wrapCryptoWaitReady } from './utils/fns.ts'
import { handleRegisterCmd } from './commands/registerGateway/register.ts'
import { handleRebootCommand } from './commands/rebootGateway/index.ts'
import { handleSubmitCmd } from './commands/submit/submit.ts'
import { handleBidCmd } from './commands/bid.ts'
import { handleDgfCmd } from './commands/dgf.ts'
import { handleEstimateMaxReward } from './commands/estimate.ts'
import { handlePurgeGatewayCommand } from './commands/purgeGateway/index.ts'
import { handlePurgeTokenCommand } from './commands/purgeToken/index.ts'
import { handleXcmTransferCommand } from './commands/xcmTransfer/index.ts'
import { handleAssetRegistrationCommand } from './commands/registerAsset/index.ts'
import { handleResetGatewayCommand } from './commands/resetGateway/index.ts'
import { handleAddSfxAbiCommand } from '@/commands/sfxABI/index.js'
import { handleEvmClaimAaddressCommand } from '@/commands/evm/claimEvmAddress.ts'
import { handleEvmGetBalanceCommand } from '@/commands/evm/getBalance.ts'
import { handleEvmTransferCommand } from '@/commands/evm/transfer.ts'
import {
  handleFastWriterCommand,
  handleMockWriterCommand,
} from './commands/fastWriter/index.ts'

const withExportMode = (program: Command) =>
  program.option('-x, --export', 'Export extrinsic data to a file')

const program = new Command()

program
  .name('t3rn CLI')
  .description('CLI for interacting with the t3rn blockchain')
  .version('0.1.0')

program
  .command('init')
  .description('Generate a config or transfer template')
  .option('-t, --transfer [file-path]', 'Generate a transfer template')
  //.option("-x, --xtransfer [file-path]", "Generate a cross-chain transfer template") // enable when agreed on XCM workflow
  .action(handleInitCmd)

withExportMode(
  program
    .command('registerGateway')
    .option('-g, --gateway <id>', 'ID of the gateway to register')
    .option(
      '-s, --slot <id>',
      'Beacon chain slot from which to register the checkpoint',
    )
    .description('Register a gateway with the circuit')
    .action(wrapCryptoWaitReady(handleRegisterCmd)),
)

withExportMode(
  program
    .command('rebootGateway')
    .argument('vendor')
    .description('Reboot a gateway')
    .action(wrapCryptoWaitReady(handleRebootCommand)),
)

program
  .command('resetGateway')
  .argument('gateway')
  .description('Reset gateway')
  .option('-f, --force', 'Force on live chain')
  .action(wrapCryptoWaitReady(handleResetGatewayCommand))

withExportMode(
  program
    .command('purgeGateway')
    .argument('gateway')
    .description('Purge a gateway')
    .option('-f, --force', 'Force on live chain')
    .action(wrapCryptoWaitReady(handlePurgeGatewayCommand)),
)

withExportMode(
  program
    .command('purgeToken')
    .argument('token')
    .description('Purge a token')
    .option('-f, --force', 'Force on live chain')
    .action(wrapCryptoWaitReady(handlePurgeTokenCommand)),
)

withExportMode(
  program
    .command('submit')
    .option('-s, --sfx <file-path>', 'Path to the sfx JSON file')
    .option(
      '-h, --headers <gateway_id>',
      'Submit the latest headers of a gateway to portal. All available finalized headers will be added.',
    )
    .description('Submit an extrinic to the t3rn blockchain')
    .action(wrapCryptoWaitReady(handleSubmitCmd)),
)

withExportMode(
  program
    .command('bid')
    .description('Bid on an execution as an Executor')
    .argument('sfxId <string>', 'sfxId of the side effect to bid on')
    .argument('amount <float>', 'bid amount')
    .action(wrapCryptoWaitReady(handleBidCmd)),
)

withExportMode(
  program
    .command('dgf')
    .description(
      'Generate side effects data with specific error modes for testing purposes on the chain.',
    )
    .option(
      '-s, --sfx <file-path>',
      'Path to the sfx JSON file',
      'transfer.json',
    )
    .option(
      '-t, --timeout <timeout>',
      'Timeout in seconds for waiting for events from the chain',
      '30',
    )
    .action(wrapCryptoWaitReady(handleDgfCmd)),
)

program
  .command('estimate')
  .requiredOption('--action <action>', 'The execution action')
  .requiredOption('--base-asset <symbol>', 'The base asset')
  .requiredOption('--target <name>', 'The target name')
  .requiredOption('--target-asset <symbol>', 'The target asset')
  .requiredOption('--target-amount <amount>', 'The amount of the target asset')
  .requiredOption(
    '--over-spend <percent>',
    'The percentage of the target amount to be used as a profit margin',
  )
  .description('Estimate the max reward for an execution')
  .action(handleEstimateMaxReward)

withExportMode(
  program
    .command('xcmTransfer')
    .description('Cross-chain transfer of assets using XCM')
    .requiredOption('--signer <string>", "The signer of the transaction')
    .requiredOption('--type <string>", "The type of XCM transfer')
    .requiredOption(
      '--endpoint <string>',
      'The RPC endpoint from which the XCM transaction will be submitted',
    )
    .requiredOption('--dest <number>", "The destination chain')
    .requiredOption('--recipient <string>", "The recipient address')
    .requiredOption('--target-asset <symbol>", "The target asset')
    .requiredOption(
      '--target-amount <amount>',
      'The amount of the target asset',
    )
    .action(handleXcmTransferCommand),
)

withExportMode(
  program
    .command('sfxAbi')
    .description('Add SFX ABI to the gateway via XDNS')
    .requiredOption('-s, --signer <string>', 'The signer of the transaction')
    .requiredOption(
      '-e, --endpoint <string>',
      'The RPC endpoint from which the XCM transaction will be submitted',
    )
    .requiredOption('-t, --target <string>', 'The destination chain')
    .requiredOption(
      '-id, --sfx-id <string>',
      "SFX ID on the destination chain - 4bytes hex string, like 'tass' or 'tran'",
    )
    .option(
      '-abi, --sfx-abi <string>',
      'SFX ABI descriptor is required - optional, but required if SFX ID is non-standard (not one of the built-in SFXs)',
    )
    .option(
      '-p, --pallet-id <number>',
      'Pallet ID on the destination chain that is responsible for generating events confirming SFX execution, e.g. 2 for Balances Pallet (must read from the runtime config',
    )
    .option(
      '-d, --purge',
      'Optional, default false. Purge SFX ABI from the gateway',
    )
    .action(handleAddSfxAbiCommand),
)

withExportMode(
  program
    .command('registerAsset')
    .description('Registering asset on AssetHub or t0rn')
    .requiredOption(
      '--endpoint <string>',
      'The RPC endpoint from which the asset will be registered',
    )
    .requiredOption('--dest <string>', 'The destination - Local/AssetHub')
    .requiredOption('--id <number>', 'The ID OF the token')
    .requiredOption('--name <string>', 'The name of the asset.')
    .requiredOption(
      '--symbol <string>',
      'The symbol of the asset - ROC/TRN/USDT',
    )
    .requiredOption(
      '--decimals <number>',
      'The amount of decimals the token has',
    )
    //.requiredOption("--sufficient <>", "Flags whether to create sufficient or non-sufficient asset")
    .action(handleAssetRegistrationCommand),
)

// example of a new command
//  pnpm writer --signer //Alice --target-account //Bob --target-asset 1000 --target-amount 100000000000 --reward-asset 0 --max-reward 40 --insurance 0.1 --speed-mode Fast --endpoint ws://localhost:9944 --dest 3333 --repeat 1 --repeat-interval 1
withExportMode(
  program
    .command('writer')
    .description(
      'Write batches of SideEffects (SFX) to the chain using the Vacuum pallet',
    )
    .requiredOption('--signer <string>', 'The signer of the transaction')
    .requiredOption(
      '--endpoint <string>',
      'The RPC endpoint from which the XCM transaction will be submitted',
    )
    .requiredOption('--dest <string>', 'The destination chain')
    .requiredOption('--source <string>', 'The source chain')
    .requiredOption('--target-asset <number>', 'Target asset ID (u32)')
    .requiredOption('--target-account <string>', 'The recipient address')
    .requiredOption(
      '--target-amount <number>',
      'The amount of the target asset',
    )
    .requiredOption('--reward-asset <number>', 'The reward asset ID (u32)')
    .requiredOption('--max-reward <number>', 'The maximum reward')
    .requiredOption('--insurance <number>', 'The insurance amount')
    .requiredOption('--speed-mode <string>', 'The speed mode')
    .option('--as-utility-batch', 'Send as a utility::batch call')
    .option('--as-sequential-tx', 'Send as a sequence of transactions')
    .option('--as-multi-sfx', 'Send as an XTX containing multiple of SFXs')
    .option('--repeat <number>', 'Repeat the transaction')
    .option(
      '--repeat-interval <number>',
      'Repeat the transaction every x seconds',
    )
    .action(handleFastWriterCommand),
)

// example of a new command
//  pnpm writer --signer //Alice --target-account //Bob --target-asset 1000 --target-amount 100000000000 --reward-asset 0 --max-reward 40 --insurance 0.1 --speed-mode Fast --endpoint ws://localhost:9944 --dest 3333 --repeat 1 --repeat-interval 1
withExportMode(
  program
    .command('mockWriter')
    .description(
      'Mock test Write batches of SideEffects (SFX) to the chain using the Vacuum pallet',
    )
    .option(
      '--repeat <number>',
      'Repeat the transaction x times as utility::batch calls',
    )
    .option(
      '--as-multi-sfx',
      'Repeat the transaction x times as utility::batch calls',
    )
    .option(
      '--as-sequential-tx',
      'Repeat the transaction x times as utility::batch calls',
    )
    .action(handleMockWriterCommand),
)

withExportMode(
  program
    .command('claimEvmAddress')
    .description('Claim EVM address for a substrate address')
    .requiredOption(
      '--endpoint <string>',
      'The RPC endpoint to transfer balance on',
    )
    .requiredOption(
      '--substrate-signer <string>',
      'The substrate account private key',
    )
    .requiredOption('--evm-signer <string>', 'The evm account private key')
    .action(handleEvmClaimAaddressCommand),
)

withExportMode(
  program
    .command('evmGetBalance')
    .description('Check EVM balance for an account.')
    .requiredOption(
      '--endpoint <string>',
      'The RPC endpoint to check the balance on',
    )
    .requiredOption('--account <string>', 'The account - EVM address')
    .action(handleEvmGetBalanceCommand),
)

withExportMode(
  program
    .command('evmTransfer')
    .description('Check EVM balance for an account.')
    .requiredOption(
      '--endpoint <string>',
      'The RPC endpoint to transfer balance on',
    )
    .requiredOption('--sender <string>', 'The sender account - EVM private key')
    .requiredOption('--receiver <string>', 'The receiver account - EVM address')
    .requiredOption('--amount <number>', 'The balance that will be transferred')
    .action(handleEvmTransferCommand),
)

program.parse(process.argv)

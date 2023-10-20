import { ProofGenerator } from './proof-generator'

const args = process.argv.slice(2)
const proofGenerator = new ProofGenerator(
  'https://sepolia.infura.io/v3/ee1e6d7e77c2415386766fa559769941',
)

if (args.length) {
  switch (args[0]) {
    case 'state':
      if (args.length !== 4) {
        console.error(
          'Usage: node script.js state <accountId> <storageId> <blockNumber>',
        )
        process.exit(1)
      }
      proofGenerator
        .generateStateProof(args[1], args[2], args[3])
        .then(console.log)
        .catch(console.error)
      break

    case 'receipt':
      if (args.length !== 2) {
        console.error('Usage: node script.js receipt <txId>')
        process.exit(1)
      }
      proofGenerator
        .generateTxReceiptProof(args[1])
        .then(console.log)
        .catch(console.error)
      break

    case 'transaction':
      if (args.length !== 2) {
        console.error('Usage: node script.js transaction <txId>')
        process.exit(1)
      }
      proofGenerator
        .generateTransactionProof(args[1])
        .then(console.log)
        .catch(console.error)
      break

    default:
      console.error(
        'Unknown command. Supported commands are state, receipt, transaction.',
      )
      process.exit(1)
  }
} else {
  console.error(
    'Please specify a command. Supported commands are state, receipt, transaction.',
  )
  process.exit(1)
}

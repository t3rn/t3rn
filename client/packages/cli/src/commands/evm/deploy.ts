import ora from 'ora'
import fs from 'fs'
import { Args } from '@/types.js'
import { validate } from '@/utils/fns.js'
import { colorLogMsg } from '@/utils/log.js'
import { ethers, ContractFactory, Wallet, JsonRpcProvider } from 'ethers'
import { EvmDeploySchema } from '@/schemas/evm.ts'

enum ConfigContractName {
    escrowGMP = 'escrowGMP',
    remoteOrder = 'remoteOrder',
    attesters = 'attesters',
    localExchange = 'localExchange',
    orderBook = 'orderBook',
    attestersPortal = 'attestersPortal',
}

export const spinner = ora()

export const handleEvmDeployCommand = async (
    _args: Args<'endpoint' | 'owner' | 'contractAbi' | 'contractBytecode' >,
) => {
    const args = validate(
        EvmDeploySchema,
        {
            ..._args,
        },
        {
            configFileName: 'EVM deploy smart contract arguments',
        },
    )

    if (!args) {
        process.exit()
    }

    spinner.text = 'Deploying EVM smart contract... \n'
    spinner.start()
    try {
        const provider = new JsonRpcProvider(args.endpoint)

        const contractOwnerWallet = new Wallet(args.owner, provider)
        const contractBytecode = fs.readFileSync(arg.contractBytecode).toString();
        const contractAbi = JSON.parse(fs.readFileSync(args.contractAbi).toString());

        const contractFactory = new ContractFactory(contractAbi, contractBytecode, contractOwnerWallet)

        // const contractERC20 = await ERC20Contract.deploy(asset, asset, { gasPrice })
        const contract = await contractFactory.deploy()
        await contract.deployed()

        spinner.stopAndPersist({
            symbol: '🎉',
            text: colorLogMsg(
                'SUCCESS',
                `${contractOwnerWallet.address} successfully uploaded smart contract at ${contract.address}`,
            ),
        })
    } catch (e) {
        spinner.fail(`Failed deploying smart contract: ${e}`)
    }

    spinner.stop()
    process.exit(0)
}
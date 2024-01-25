import { z } from 'zod'

export const EvmGetBalanceSchema = z.object({
    endpoint: z.string({
        invalid_type_error: 'Endpoint must be a string',
        required_error: 'Endpoint is required',
    })
        .startsWith('ws://')
        .or(z.string().startsWith('wss://')),
    account: z.string({
        invalid_type_error: 'Account must be a string',
        required_error: 'Account is required',
    })
        .regex(/^0x[a-fA-F0-9]{40}$/, { message: 'Account must be a vlaid EVM address' }),
})

export const EvmTransferSchema = z.object({
    endpoint: z.string({
        invalid_type_error: 'Endpoint must be a string',
        required_error: 'Endpoint is required',
    })
        .startsWith('ws://')
        .or(z.string().startsWith('wss://')),
    sender: z.string({
        invalid_type_error: 'Sender must be a string',
        required_error: 'Sender is required',
    })
        .regex(/^0x[a-fA-F0-9]{40}$/, { message: 'Sender must be a vlaid EVM address' }),
    receiver: z.string({
        invalid_type_error: 'Receiver must be a string',
        required_error: 'Receiver is required',
    })
        .regex(/^0x[a-fA-F0-9]{40}$/, { message: 'Receiver must be a vlaid EVM address' }),
    amount: z.number({
        invalid_type_error: 'Amount must be a number',
        required_error: 'Amount is required',
    })
        .positive(),

})

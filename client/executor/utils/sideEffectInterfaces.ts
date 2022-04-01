export type SideEffect = {
    sender: any,
    xtxId: any,
    target: any,
    prize: any,
    orderedAt: any,
    encodedAction: any,
    signature: any,
    enforceExecutioner: any,
    decodedArgs: Transfer,
    encodedArgs: any,
}

export type Transfer = {
    from: any,
    to: any,
    amount: any
}

export const deconstruct = (args: any): SideEffect => {
    let sideEffect: SideEffect = {
        sender: args[0],
        xtxId: args[1],
        target: args[2][0].target,
        prize: args[2][0].prize,
        orderedAt: args[2][0].orderedAt,
        encodedAction: args[2].encodedAction,
        signature: args[2][0].signature,
        enforceExecutioner: args[2][0].enforceExecutioner,
        decodedArgs: deconstructArgs(args[2][0].encodedArgs),
        encodedArgs: args[2][0].encodedArgs,
    }

    return sideEffect;
}

const deconstructArgs = (args: any): Transfer => {
    let transfer: Transfer = {
        from: args[0],
        to: args[1],
        amount: args[2]
    }
    return transfer
} 
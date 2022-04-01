export type SideEffect = {
    sender: any,
    xtxId: any,
    target: any,
    prize: any,
    encodedAction: any,
    signature: any,
    enforceExecutioner: any,
    encodedArgs: Transfer
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
        encodedAction: args[2].encodedAction,
        signature: args[2][0].signature,
        enforceExecutioner: args[2][0].enforceExecutioner,
        encodedArgs: deconstructArgs(args[2][0].encodedArgs)
    }

    return sideEffect;
}

const deconstructArgs = (args: any): Transfer => {
    let transfer: Transfer = {
        from: args[0],
        to: args[1].toHuman(),
        amount: args[2].toHuman()
    }
    return transfer
} 
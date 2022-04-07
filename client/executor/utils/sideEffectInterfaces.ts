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

// {
//     target: target, // [97, 98, 99, 100] -> registered for testing, "abcd" in bytes
//     prize: 0,
//     ordered_at: 0,
//     encoded_action: [116, 114, 97, 110], //tran
//     encoded_args: [keyring.alice.address, keyring.charlie.address, [amount, 0, 0, 0, 0, 0, 0, 0]],
//     signature: [],
//     enforce_executioner: '',
// }

export type Transfer = {
    from: any,
    to: any,
    amount: any
}

export const deconstruct = (args: any): SideEffect => {
    // console.log("SE:", args[2][0].enforceExecutioner.toHuman());
    let sideEffect: SideEffect = {
        sender: args[0],
        xtxId: args[1],
        target: args[2][0].target,
        prize: args[2][0].prize,
        orderedAt: args[2][0].orderedAt,
        encodedAction: args[2][0].encodedAction,
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
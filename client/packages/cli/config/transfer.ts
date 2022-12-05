export default {
    sideEffects: [
        {
            target: "roco",
            type: "tran",
            to: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
            amount: "0.01", // in ROC
            insurance: "0.1", // in TRN
            reward: "4", // in TRN
        },
        {
            target: "roco",
            type: "tran",
            to: "5EoHBHDBNj61SbqNPcgYzwHXY1xAroduRP3M99iSMZ8kwvgp",
            amount: "0.05", // in ROC
            insurance: "0.8", // in TRN
            reward: "10", // in TRN
        },
    ],
    sequential: false,
}
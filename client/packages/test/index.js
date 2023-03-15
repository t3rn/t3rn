import "@t3rn/types";
import { createType } from "@t3rn/types";
import { ssz } from "@lodestar/types";
let sfx = createType("T3rnTypesSfxSideEffect", {
    target: "roco",
    maxReward: 1000000,
    insurance: 100000,
    action: "tran",
    encodedArgs: ["0x0", "0x1"],
    signature: "",
    enforceExecutor: "",
    rewardAssetId: 1,
});
console.log("createType", createType);
console.log("ssz", ssz);
console.log("sfx", sfx.toHuman());

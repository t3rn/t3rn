import { e } from "@t3rn/sdk/dist/decoder-61ad6234";
import { EVMRelayer } from "../relayer";

const sfx = {
  "sideEffects": [
    {
      "target": "sepl",
      "maxReward": "40",
      "insurance": "0.1",
      "action": "tran",
      "encodedArgs": [
        {
          "from": "5Hmf2ARKQWr2RXLYUuZRN2HzEoDLVUGquhwLN8J7nsRMYcGQ",
          "to": "5Hmf2ARKQWr2RXLYUuZRN2HzEoDLVUGquhwLN8J7nsRMYcGQ"
        }
      ],
      "signature": "0x",
      "enforceExecutor": null,
      "rewardAssetId": null
    }
  ],
  "speed_mode": "Fast"
};

const evm = new EVMRelayer();

evm.setup("0x")

// evm.send(sfx).then(console.log).catch(console.error);


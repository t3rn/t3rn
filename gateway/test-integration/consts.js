const BN = require("bn.js");

const WSURL = "ws://127.0.0.1:9944";
const DOT = new BN("1000000000000000");
const CREATION_FEE = DOT.muln(200);
const GAS_REQUIRED = 100000000000;
const GAS_LIMIT = 0xffffffff; // u32::MAX
const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
const CHARLIE = "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y";
const DAVE = "126TwBzBM4jUEK2gTphmW4oLoBWWnYvPp8hygmduTr4uds57";


module.exports = {
	BN, WSURL, DOT, CREATION_FEE, GAS_LIMIT, GAS_REQUIRED, ALICE, BOB, CHARLIE, DAVE,
};

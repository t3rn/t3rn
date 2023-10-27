"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
var assert = require("assert");
var path = require('path');
var sdk_1 = require("@t3rn/sdk");
var ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
var BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
function checkEvent(api) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            return [2 /*return*/, new Promise(function (resolve) {
                    var eventSection = 'xdns';
                    var eventName = 'GatewayRecordStored';
                    api.query.system.events(function (events) {
                        var expectedEvents = events
                            .toHuman()
                            .filter(function (event) { return event.event.section === eventSection && event.event.method === eventName && event.event.data[0] === 'roco'; });
                        if (expectedEvents.length > 0) {
                            console.log("\u2705 Event ".concat(eventSection, ".").concat(eventName, " emitted for ").concat(expectedEvents[0].event.data));
                            resolve(1);
                        }
                        else {
                            console.log("\u23F3 Event ".concat(eventSection, ".").concat(eventName, " for roco not yet emitted."));
                        }
                    });
                })];
        });
    });
}
function run(nodeName, networkInfo, args) {
    return __awaiter(this, void 0, void 0, function () {
        var _a, wsUri, userDefinedTypes, api, keyring, signer;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    _a = networkInfo.nodesByName[nodeName], wsUri = _a.wsUri, userDefinedTypes = _a.userDefinedTypes;
                    return [4 /*yield*/, zombie.connect(wsUri, userDefinedTypes)];
                case 1:
                    api = _b.sent();
                    try {
                        keyring = new sdk_1.Keyring({ type: "sr25519" });
                        signer = keyring.addFromUri("//Alice");
                        //await api.tx.assets.createAsset()
                    }
                    catch (err) {
                        console.error(err);
                        process.exit(1);
                    }
                    return [2 /*return*/, 1
                        //const result = await checkEvent(api);
                        //return result;
                        /*
                           const command = `pnpm cli xcmTransfer
                            --signer "//Bob"
                            --type "relay"
                            --endpoint "ws://127.0.0.1:9933"
                            --dest "3333"
                            --recipient "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
                            --target-asset "ROC"
                            --target-amount 2000000000000'
                           `
                           const cliPath = path.join(__dirname, '../../../client/packages/cli')
                    
                           try {
                               exec(command, { cwd: cliPath }, (error) => {
                                 if (error) {
                                   console.error(`Error executing command "${command}": ${error}`);
                                   process.exit(1)
                                 }
                               });
                           } catch (err) {
                               console.error(err);
                               process.exit(1)
                           }
                    
                           const result = await checkEvent(api);
                           return result;
                       }
                       */
                    ];
            }
        });
    });
}
module.exports = { run: run };

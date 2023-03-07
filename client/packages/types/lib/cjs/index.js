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
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.entriesByIds = exports.keysOf = exports.createType = exports.registry = void 0;
require("./augment/types-lookup");
require("./augment/registry");
require("./augment/augment-api");
const types_1 = require("@polkadot/types");
const lookup_1 = __importDefault(require("./augment/lookup"));
exports.registry = new types_1.TypeRegistry();
exports.registry.register(lookup_1.default);
function createType(typeName, value) {
    return exports.registry.createType(typeName, value);
}
exports.createType = createType;
function keysOf(typeName) {
    return exports.registry.createType(typeName).defKeys;
}
exports.keysOf = keysOf;
function entriesByIds(apiMethod) {
    return __awaiter(this, void 0, void 0, function* () {
        const entries = (yield apiMethod.entries()).map(([storageKey, value]) => [
            storageKey.args[0],
            value,
        ]);
        return entries.sort((a, b) => a[0].toNumber() - b[0].toNumber());
    });
}
exports.entriesByIds = entriesByIds;

import { readSfxFile, submitSfx } from "@/commands/submit/sfx.ts";
import { validate } from "../fns.ts";
import { ExtrinsicSchema, Extrinsic, SpeedMode } from "@/schemas/extrinsic.ts";
import * as fs from "fs";
import { error } from "console";

enum ErrorMode {
    NoBidders = "NoBidders",
    ConfirmationTimeout = "ConfirmationTimeout",
    InvalidProof = "InvalidProof",
    InvalidExecutionValidProof = "InvalidExecutionValidProof",
}

// interface TransactionArgsInterface {
//     sideEffects: sideEffectInterface;
//     speed_mode: SpeedMode;
// }

// interface sideEffectInterface {
//     target: string,             // "roco",
//     maxReward: string,          // "40",
//     insurance: string,          // "0.1", // in TRN
//     action: string,             // "tran",
//     encondedArgs: {
//         from: string,           // "5Hmf2ARKQWr2RXLYUuZRN2HzEoDLVUGquhwLN8J7nsRMYcGQ",
//         to: string,             // "5Hmf2ARKQWr2RXLYUuZRN2HzEoDLVUGquhwLN8J7nsRMYcGQ",
//     }
//     signature: string,          // "0x",
//     enforceExecutor: string,    // null,
//     rewardAssetId: string,      // "40", // in TRN
// }


/**
 * Process the get, injection and save of the SFX.
 * 
 * @param args arguments passed to the CLI
 * @param sfxFile file containing the SFX
 */
// const getSfx = async (args: Args<"sfx" | "headers">, sfxFile: string, errorMode: ErrorMode) => {
const processSfx = async (sfxFile: string, errorMode: ErrorMode) => {
    // Get the extrinsic from the file
    const extrinsic = getExtrinsic(sfxFile);

    // Attach the error on the SFX
    injectErrorMode(extrinsic, errorMode);

    // Save the SFX as a json file
    saveToJson(extrinsic, errorMode);

    // TODO: should we submit it?
    // submitSfx(extrinsic, true); // nothing is returned so, how do I check it works?

    // test it with bidding
}

/**
 * Get the SFX and validate it.
 * 
 * @param sfxFile file to read the extrinsic from
 * @returns the validated extrinsic
 */
const getExtrinsic = (sfxFile: string) => {
    // Read from file the extrinsic
    const unvalidatedExtrinsic = readSfxFile(sfxFile);
    maybeExit(unvalidatedExtrinsic);

    // Validate it
    const extrinsic: Extrinsic = validate(ExtrinsicSchema, unvalidatedExtrinsic, {
        configFileName: sfxFile,
    });
    maybeExit(extrinsic);

    return extrinsic
}


/**
 * Modify the SFX buy injecting the error mode in the signature field.
 * 
 * What is accepted is a transaction args object, which contains the
 * side effect and the speed mode.
 * 
 * @param sfx 
 * @param errorMode 
 */
const injectErrorMode = (extrinsic: Extrinsic, errorMode: ErrorMode) => {
    // too simple that you want to kill me
    extrinsic.sideEffects.signature = errorMode
    console.log("✅ Succesfully injected the error in the SFX!");
}


/**
 * Check if `value` exists or not, and exit the process accordingly.
 * 
 * @param value anything that can be checked if exists or not
 */
const maybeExit = (value: any) => {
    value ? process.exit(0) : process.exit(1)
}

/**
 * Save the extrinsic as a json file.
 * The default location is `./sfx_with_error_modes`.
 * 
 * @param sfx The extrinsic to be saved
 * @param folder The place to store the extrinsic
 */
const saveToJson = (sfx: Extrinsic, errorMode: ErrorMode, folder: string = "./sfx_with_error_modes") => {
    if (!fs.existsSync(folder)) {
        fs.mkdirSync(folder);
    }
    fs.writeFileSync(`${folder}/sfx_${errorMode}.json`, JSON.stringify(sfx));
    console.log("✅ Succesfully saved the SFX as a json file!")
}


/**
 * Batch-create all the SFX with all the possible different errors.
 * 
 */
const batchErrorModes = () => {
    for (const errorMode in ErrorMode) {
        processSfx("./sfx.json", ErrorMode[errorMode]);
    }
}

batchErrorModes();

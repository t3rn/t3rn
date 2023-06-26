import dotenv from "dotenv";
import path from "path";

// Parsing the env file.
dotenv.config({ path: path.resolve(__dirname, "./.env") });

interface Config {
    SEPOLIA: string,
}

export const getConfig = (): Config => {
    return {
        SEPOLIA: process.env.SEPOLIA_ENDPOINT ? process.env.SEPOLIA_ENDPOINT : ""
    }
}

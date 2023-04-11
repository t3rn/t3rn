import { Instance } from "./index"

async function main() {
    const instance = new Instance()
    await instance.setup(process.env.EXECUTOR)
}

main()

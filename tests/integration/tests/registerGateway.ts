const { exec } = require('child_process');

const wait = (waitInSecs: number) => {
    console.log(`Waiting ${waitInSecs} seconds after registering the gateway!`)
    return new Promise(resolve => setTimeout(resolve, waitInSecs * 1000));
};

const execute = async (command: string, waitInSec: number) => {
    console.log("Executing: ", command)
    return new Promise((resolve, reject) => {
        exec(
            `cd ../../../client/packages/cli && ts-node index.ts ${command}`,
            (error: any, stdout: string, stderr: string) => {
                console.log(stdout)
                if (error) {
                    reject(error);
                } else {
                    resolve(stdout);
                }
            });
    })
        .finally(() => {
            return wait(waitInSec)
        })
}

const registerGateway = async (context, ...args) => {
    await execute("register roco", 20);
}

export default registerGateway
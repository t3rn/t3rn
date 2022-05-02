export const goCatch= async (promise) => {
    try {
        const result = await promise
        return [result, null]
    } catch (err) {
        return [null, err]
    }
}

// // logging colors from chalk
// export const colors = [
//     "redBright",
//     "greenBright",
//     "yellowBright",
//     "blueBright",
//     "magentaBright",
//     "cyanBright",
//     "whiteBright",
//     "red",
//     "green",
//     "yellow",
//     "blue",
//     "magenta",
//     "cyan",
//     "white",
//     "grey",
// ]
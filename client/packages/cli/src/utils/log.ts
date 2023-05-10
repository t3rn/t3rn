import chalk from "chalk"

type Log = "INFO" | "SUCCESS" | "WARN" | "ERROR"

export const mapLogColor = (type: Log) => {
  switch (type) {
    case "INFO":
      return ["blue", "bgBlue"]
    case "SUCCESS":
      return ["green", "bgGreen"]
    case "WARN":
      return ["yellow", "bgYellow"]
    case "ERROR":
      return ["red", "bgRed"]
  }
}

export const log = (type: Log, msg: string) => {
  console.log(fmtLog(type, msg))
}

export const colorLogMsg = (type: Log, msg: string) => {
  const [fgColor] = mapLogColor(type)
  return chalk[fgColor](msg)
}

export const fmtLog = (type: Log, msg: string) => {
  const [, bgColor] = mapLogColor(type)
  return `${chalk.black.bold[bgColor](type)} ${colorLogMsg(type, msg)}`
}

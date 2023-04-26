import { describe } from "@jest/globals"
import chalk from "chalk"
import { colorLogMsg, fmtLog, log, mapLogColor } from "../log.ts"

describe("mapLogColor", () => {
  test("should return blue and bgBlue for INFO", () => {
    expect(mapLogColor("INFO")).toEqual(["blue", "bgBlue"])
  })

  test("should return green and bgGreen for SUCCESS", () => {
    expect(mapLogColor("SUCCESS")).toEqual(["green", "bgGreen"])
  })

  test("should return yellow and bgYellow for WARN", () => {
    expect(mapLogColor("WARN")).toEqual(["yellow", "bgYellow"])
  })

  test("should return red and bgRed for ERROR", () => {
    expect(mapLogColor("ERROR")).toEqual(["red", "bgRed"])
  })
})

describe("colorLogMsg", () => {
  test("should return blue for INFO", () => {
    expect(colorLogMsg("INFO", "test")).toEqual(chalk.blue("test"))
  })
})

const expectedOutput = `${chalk.black.bold.bgBlue("INFO")} ${chalk.blue(
  "test"
)}`

describe("fmtLog", () => {
  test("should return blue and bgBlue for INFO", () => {
    expect(fmtLog("INFO", "test")).toEqual(expectedOutput)
  })
})

describe("log", () => {
  test("should call console.log", () => {
    console.log = jest.fn()
    log("INFO", "test")
    expect(console.log).toHaveBeenCalledWith(expectedOutput)
  })
})

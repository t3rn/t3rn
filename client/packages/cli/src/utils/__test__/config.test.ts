import * as fs from "fs"
import {
  checkForConfigFile,
  getConfig,
  parseConfigFile,
} from "@/utils/config.ts"
import setup from "@/templates/setup.ts"
import { Config } from "@/schemas/setup.ts"

jest.mock("fs")

describe("checkConfigFile", () => {
  test("should return true if config file exists", () => {
    const existsSync = jest.spyOn(fs, "existsSync")
    existsSync.mockReturnValue(true)

    expect(checkForConfigFile()).toBe(true)
  })

  test("should return false if config file does not exist", () => {
    const existsSync = jest.spyOn(fs, "existsSync")
    existsSync.mockReturnValue(false)
    console.log = jest.fn()

    expect(checkForConfigFile()).toBe(false)
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("ERROR"))
  })
})

const config = JSON.stringify(setup)

describe("parseConfigFile", () => {
  test("should return config object if config file exists", () => {
    const readFileSync = jest.spyOn(fs, "readFileSync")
    readFileSync.mockReturnValue(config)

    expect(parseConfigFile("./")).toEqual(setup)
  })

  test("should return undefined if config file does not exist", () => {
    const readFileSync = jest.spyOn(fs, "readFileSync")
    readFileSync.mockReturnValue(undefined)
    console.log = jest.fn()

    expect(parseConfigFile("./")).toBe(undefined)
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("ERROR"))
  })
})

describe("getConfig", () => {
  beforeEach(() => {
    console.log = jest.fn()
  })

  test("should return config object if config file exists", () => {
    const existsSync = jest.spyOn(fs, "existsSync")
    existsSync.mockReturnValue(true)
    const readFileSync = jest.spyOn(fs, "readFileSync")
    readFileSync.mockReturnValue(config)

    expect(getConfig()).toEqual(setup)
  })

  test("should return undefined if config file does not exist", () => {
    const existsSync = jest.spyOn(fs, "existsSync")
    existsSync.mockReturnValue(false)

    expect(getConfig()).toBe(undefined)
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("ERROR"))
  })

  test("should return undefined if config file is invalid", () => {
    const badConfig: Config = JSON.parse(config)
    badConfig.circuit = {}

    const existsSync = jest.spyOn(fs, "existsSync")
    existsSync.mockReturnValue(true)
    const readFileSync = jest.spyOn(fs, "readFileSync")
    readFileSync.mockReturnValue(JSON.stringify(badConfig))

    expect(getConfig()).toBe(undefined)
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("ERROR"))
  })
})

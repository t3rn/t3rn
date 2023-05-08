import * as fs from "fs"
import { describe, test, expect, jest } from "@jest/globals"
import { initConfigFile, initTransferFile } from "../init.ts"

jest.mock("fs")

describe("initConfigFile", () => {
  beforeEach(() => {
    console.log = jest.fn()
  })

  test("should generate config template", () => {
    const writeFileSync = jest.spyOn(fs, "writeFileSync")
    writeFileSync.mockReturnValue()

    initConfigFile(true)
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("SUCCESS"))
  })

  test("should genrate transfer template", () => {
    const writeFileSync = jest.spyOn(fs, "writeFileSync")
    writeFileSync.mockReturnValue()

    initTransferFile(true)
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("SUCCESS"))
  })
})

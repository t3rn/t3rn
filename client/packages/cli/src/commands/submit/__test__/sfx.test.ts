import { describe, expect, test } from "@jest/globals"
import fs from "fs"
import transferTemplate from "@/templates/transfer.ts"
import { readSfxFile } from "../sfx.ts"

jest.mock("fs")

describe("readSfxFile", () => {
  test("should read file and return an object", () => {
    jest.spyOn(fs, "existsSync").mockReturnValueOnce(true)
    jest
      .spyOn(fs, "readFileSync")
      .mockReturnValueOnce(JSON.stringify(transferTemplate))

    const sideEffect = readSfxFile("transfer.json")
    expect(sideEffect).toEqual(transferTemplate)
  })

  test("should return undefined if file does not exist", () => {
    jest.spyOn(fs, "existsSync").mockReturnValueOnce(false)
    console.log = jest.fn()

    const sideEffect = readSfxFile("transfer.json")
    expect(sideEffect).toBeUndefined()
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("ERROR"))
  })
})

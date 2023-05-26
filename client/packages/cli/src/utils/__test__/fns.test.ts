import { describe, expect, test } from "@jest/globals"
import { z } from "zod"
import { validate, greet } from "../fns.ts"

describe("greet", () => {
  test("should return a greeting", () => {
    console.log = jest.fn()
    greet()
    expect(console.log).toBeCalled()
  })
})

const Schema = z.object({
  tokenId: z.string(),
  tokenSymbol: z.string(),
  tokenDecimals: z.number(),
})

describe("validate", () => {
  test("should pass validation if a valid value is passed", () => {
    const value = { tokenId: "0x123", tokenSymbol: "ABC", tokenDecimals: 18 }
    expect(validate(Schema, value)).not.toBeUndefined()
    expect(validate(Schema, value)).toEqual(value)
  })

  test("should fail validation if an invalid value is passed", () => {
    const value = { tokenId: 12, tokenSymbol: "ABC", tokenDecimals: 18.5 }
    console.log = jest.fn()
    expect(validate(Schema, value)).toBeUndefined()
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("ERROR"))
  })
})

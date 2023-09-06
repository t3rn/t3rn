import { beforeAll, describe } from "@jest/globals"
import { spinner } from "../reboot.ts"
import { fetchLatestAuthoritySetUpdateBlock } from "../vendors/substrate.ts"

// @TODO: node fetch is an esm module. We need to update and other tests to support ESM module testing
// for now lets skip this test

jest.mock("node-fetch", () => ({
  default: jest.fn(),
}))

describe("fetchLatestAuthoritySetUpdateBlock", () => {
  beforeAll(() => {
    console.log = jest.fn()
  })

  afterAll(() => {
    jest.resetAllMocks()
  })

  test.skip("should return the latest authority set update block", async () => {
    const fetch = await import("node-fetch")
    const mockFetch = jest.spyOn(fetch, "default")

    mockFetch.mockResolvedValueOnce({
      status: 200,
      json: () =>
        Promise.resolve({
          data: {
            events: [
              {
                block_num: 1,
              },
            ],
          },
        }),
    } as never)

    const block = await fetchLatestAuthoritySetUpdateBlock(
      "http://localhost:8080",
    )
    expect(block).toEqual(1)
  })

  test.skip("should return undefined if an error occurs", async () => {
    const fetch = await import("node-fetch")
    const mockFetch = jest.spyOn(fetch, "default")

    mockFetch.mockRejectedValueOnce(new Error("error"))
    jest.spyOn(spinner, "fail")

    const block = await fetchLatestAuthoritySetUpdateBlock(
      "http://localhost:8080",
    )
    expect(block).toBeUndefined()
    expect(spinner.fail).toHaveBeenCalled()
  })
})

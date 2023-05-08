import fetch from "node-fetch"
import { beforeAll, describe } from "@jest/globals"
import { spinner, fetchLatestAuthoritySetUpdateBlock } from "../gateway.ts"

jest.mock("node-fetch")

describe("fetchLatestAuthoritySetUpdateBlock", () => {
  beforeAll(() => {
    console.log = jest.fn()
  })

  afterAll(() => {
    jest.resetAllMocks()
  })

  test("should return the latest authority set update block", async () => {
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
      "http://localhost:8080"
    )
    expect(block).toEqual(1)
  })

  test("should return undefined if an error occurs", async () => {
    const mockFetch = jest.spyOn(fetch, "default")
    mockFetch.mockRejectedValueOnce(new Error("error"))
    jest.spyOn(spinner, "fail")

    const block = await fetchLatestAuthoritySetUpdateBlock(
      "http://localhost:8080"
    )
    expect(block).toBeUndefined()
    expect(spinner.fail).toHaveBeenCalled()
  })
})

import { Args } from "@/types.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { getConfig } from "@/utils/config.ts"
import { colorLogMsg } from "@/utils/log.ts"
import { createType } from "@t3rn/types"
import ora from "ora"

const spinner = ora()

export const handleBidCmd = async (
  sfxId: string,
  amount: number,
  options: Args<"export">
) => {
  const config = getConfig()
  if (!config) {
    process.exit(1)
  }

  const { circuit, sdk } = await createCircuitContext(Boolean(options.export))
  const bidArgs = bid(circuit, sdk, sfxId, amount)

  spinner.start(colorLogMsg("INFO", `Bidding on side effect ${sfxId}...`))

  try {
    const transaction = circuit.tx.circuit.bidSfx(
      bidArgs.sfxId,
      bidArgs.bidAmount
    )
    await sdk.circuit.tx.signAndSendSafe(transaction)
    spinner.stopAndPersist({
      symbol: "🚩",
      text: colorLogMsg(
        "SUCCESS",
        `Bid successfully placed on side effect ${sfxId} for ${amount}`
      ),
    })
    process.exit(0)
  } catch (error) {
    spinner.fail(colorLogMsg("ERROR", error))
    process.exit(1)
  }
}

const bid = (
  circuit: Awaited<ReturnType<typeof createCircuitContext>>["circuit"],
  sdk: Awaited<ReturnType<typeof createCircuitContext>>["sdk"],
  sfxId: string,
  amount: number
) => {
   
  return {
    sfxId: circuit.createType("SideEffectId", sfxId),
    // @ts-ignore - augmeneted type from @polkadot/types
    bidAmount: createType("u128", sdk.circuit.floatToBn(amount)),
  }
}

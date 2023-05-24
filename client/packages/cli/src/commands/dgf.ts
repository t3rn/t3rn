import { Args } from "@/types.ts"
import { createCircuitContext } from "@/utils/circuit.ts"
import { getConfig } from "@/utils/config.ts"
import { colorLogMsg } from "@/utils/log.ts"
import { createType } from "@t3rn/types"
import ora from "ora"
import { ErrorMode, batchErrorModes } from "@/utils/dgf/sfx_creation.ts"

const spinner = ora()

export const handleDgfCmd = async (
  sfxId: string,
  options: Args<"export">
) => {
  const config = getConfig()
  if (!config) {
    process.exit(1)
  }

  const { circuit, sdk } = await createCircuitContext(Boolean(options.export))

  spinner.start(colorLogMsg("INFO", 'Generating data...'))

  batchErrorModes()
}

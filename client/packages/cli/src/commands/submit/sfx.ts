import "@t3rn/types"
import ora from "ora"
import { validate } from "@/utils/fns.ts"
import { colorLogMsg } from "@/utils/log.ts"
import { ExtrinsicSchema } from "@/schemas/extrinsic.ts"
import { readSfxFile, submitSfx } from "@/utils/sfx.ts"

export const spinner = ora()

export const handleSubmitSfxCmd = async (
  sfxFile: string,
  exportMode: boolean,
) => {
  const unvalidatedExtrinsic = readSfxFile(sfxFile)

  if (!unvalidatedExtrinsic) {
    process.exit(1)
  }

  const extrinsic = validate(ExtrinsicSchema, unvalidatedExtrinsic, {
    configFileName: sfxFile,
  })

  if (!extrinsic) {
    process.exit(1)
  }

  spinner.text = "Submitting extrinsic..."
  spinner.start()

  try {
    const submissionHeight = await submitSfx(extrinsic, exportMode)
    spinner.stopAndPersist({
      symbol: "🚀",
      text: colorLogMsg(
        "SUCCESS",
        `Extrinsic submitted at block #${submissionHeight}`,
      ),
    })
    process.exit(0)
  } catch (e) {
    spinner.fail(`Extrinsic submission failed: ${e}`)
    process.exit(1)
  }
}

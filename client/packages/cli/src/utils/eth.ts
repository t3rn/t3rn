import { EthActions, EthSpeedModes } from "@t3rn/sdk/price-estimation"
import { SpeedMode, SpeedModes } from "@/schemas/extrinsic.ts"
import { SideEffectAction, SideEffectActions } from "@/schemas/sfx.ts"

export const mapSfxSpeedModeToEthSpeedMode = (speedMode: SpeedMode) => {
  switch (speedMode) {
    case SpeedModes.Fast:
      return EthSpeedModes.Fast
    case SpeedModes.Rational:
    case SpeedModes.Finalized:
      return EthSpeedModes.Standard
  }
}

export const mapSfxActionToEthAction = (action: SideEffectAction) => {
  switch (action) {
    case SideEffectActions.Transfer:
    default:
      return EthActions.Transfer
  }
}

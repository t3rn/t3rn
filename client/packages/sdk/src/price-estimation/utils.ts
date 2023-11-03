import { Target } from "price-estimation"

export class NativeAssetMap {
  static DEFAULT_ETH_TARGET_NATIVE_ASSET = "eth"
  static DEFAULT_SUBSTRATE_TARGET_NATIVE_ASSET = "aca"

  private static map: Record<Target, string | null> = {
    eth: "eth",
    dot: "dot",
    sepl: null,
    roco: null,
    t0rn: null,
  }

  public static getFor(target: Target) {
    const value = NativeAssetMap.map[target]
    if (value) return value
    if (getChainVendor(target) === "substrate") {
      return NativeAssetMap.DEFAULT_SUBSTRATE_TARGET_NATIVE_ASSET
    }
    return NativeAssetMap.DEFAULT_ETH_TARGET_NATIVE_ASSET
  }

  public static setFor(target: Target, value: string) {
    NativeAssetMap.map[target] = value
  }
}

const getChainVendor = (target: string) => {
  switch (target) {
    case "eth":
    case "sepl":
      return "eth"
    default:
      return "substrate"
  }
}

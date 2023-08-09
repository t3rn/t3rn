export const targetToNativeAssetMap: Record<string, string | null> = {
  eth: "eth",
  dot: "dot",
  sepl: null,
  roco: null,
  t0rn: null
}

export const DEFAULT_ETH_TARGET_NATIVE_ASSET = "eth"
export const DEFAULT_SUBSTRATE_TARGET_NATIVE_ASSET = "aca"

export const getTargetNativeAsset = (target: string) => {
  const value = targetToNativeAssetMap[target]
  if (value) return value
  if (getChainVendor(target) === "substrate") {
    return DEFAULT_SUBSTRATE_TARGET_NATIVE_ASSET
  }
  return DEFAULT_ETH_TARGET_NATIVE_ASSET
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

const { u8aToHex, isHex, isString, isNumber } = require("@polkadot/util")
import * as BN from "bn.js"

/**
 * This class is used for doing amount conversions of different types. When dealing with circuit, there are three main encodings are used for representing amounts:

 * - `LittleEndian` - Used by Substrate as part of the SCALE encoding. The amount is LittleEndian encoded
 * - `Float` - Floats are the human-readable format of amounts. This is very user facing.
 * - `Integer` - In combination with decimals, used to represent amounts. These are represented as TS number or BN object
 *
 * This class enables the conversion between these different types. The gateway class heavily relies on this class, passing parameters like decimals and value types out of the box.
 */

export class AmountConverter {
  value: BN
  decimals: number
  valueTypeSize: number

  /**
   * Construct an AmountConverter, ensuring that only integers or LittleEndian values are passed.
   *
   * @param args - The arguments to convert
   * @param args.value - The arguments to convert. Two types are possible with different encodings:
   * - Integer: BN, string or number
   * - LittleEndian: hex string
   *
   * @param args.decimals - The decimals of the target. Default: 12
   * @param args.valueTypeSize - The value type size in bytes. Default: 16 (u128)
   *
   *
   *
   * ```typescript
   * new AmountConverter({
   *   100000,
   *   decimals: 12,
   *   valueTypeSize: 16
   * })
   * .toFloat()
   *
   * ```
   */

  constructor(args: {
    value?: number | BN | string
    decimals?: number
    valueTypeSize?: number
  }) {
    // Set defaults
    if (!args.decimals) args.decimals = 12
    if (!args.valueTypeSize) args.valueTypeSize = 16

    this.decimals = args.decimals
    this.valueTypeSize = args.valueTypeSize

    if (args.value) {
      // interpret string values correctly
      if (isString(args.value)) {
        if (isHex(args.value)) {
          this.value = fromLeEncoding(args.value as string)
        } else {
          this.value = new BN.BN(args.value, 10)
        }
      } else if (isNumber(args.value)) {
        if (Math.floor(args.value as number) !== args.value) {
          throw new Error(
            "AmountConverter: Float values not supported! Please convert to Integer!",
          )
        } else {
          this.value = new BN.BN(args.value, 10)
        }
      } else {
        this.value = args.value as BN
      }
    }
  }

  /**
   * Convert the initialzed value into a float
   */

  toFloat(): number {
    return bnToFloat(this.toBn(), this.decimals)
  }

  /**
   * Convert the initialized value into BN
   */

  toBn(): BN {
    return this.value as BN
  }

  /**
   * Convert a float parameter into BN, using the set decimals
   * @param value - The value to convert
   */

  floatToBn(value: number): BN {
    return floatToBn(value, this.decimals)
  }

  /**
   * Convert the initialized value into a LittleEndian byte array
   */

  toLeArray(): number[] {
    return toLeEncoding(this.value as BN, this.valueTypeSize)
  }

  /**
   * Convert the initialized value into LittleEndian encoded hex string
   */

  toLeHex(): string {
    return u8aToHex(this.toLeArray())
  }
}

/**
 * Converts hex LittleEndian to BN
 * @param hex - The hex string to convert
 */

export const fromLeEncoding = (number: string): BN => {
  return new BN.BN(number.split("0x")[1], 16, "le")
}

/**
 * Encode number to LE array
 * @param number - The number to encode
 * @param valueTypeSize - The size of the value type in bytes
 */

export const toLeEncoding = (
  number: BN,
  valueTypeSize: number | undefined,
): number[] => {
  if (valueTypeSize === undefined) {
    valueTypeSize = 16 as number
  }
  return new BN.BN(number).toArray("le", valueTypeSize)
}

/**
 * Converts decimal number to uint as BigNumber. BN.js takes care of correct rounding here
 * @param number - The number to convert
 * @param decimals - The decimals of the number
 */

export const floatToBn = (number: number, decimals: number): BN => {
  return new BN.BN(number * Math.pow(10, decimals))
}

/**
 * Converts uint to a human readable decimal
 * @param number - The number to convert
 * @param decimals - The decimals of the number
 */

export const bnToFloat = (number: BN, decimals: number): number => {
  return number.toNumber() / Math.pow(10, decimals)
}

/**
 * Generate the optional insurance construct.
 * As the resulting value is only used on t3rn, we rely on the correct default values here
 * @param insurance - The insurance amount
 */

export const optionalInsurance = (
  insurance: number | BN | string,
  reward: number | BN | string,
) => {
  const encodedInsurance = new AmountConverter({
    value: insurance,
  }).toLeArray()
  const encodedReward = new AmountConverter({ value: reward }).toLeArray()
  return u8aToHex([...encodedInsurance, ...encodedReward])
}

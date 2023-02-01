import { u8aToHex, isHex, isString, isNumber } from "@polkadot/util";
import BN from "bn.js";
import Big from 'big.js';

/**
 * This class is used for doing amount conversions of different types. When dealing with circuit, there are three main encodings are used for representing amounts:

 * - `LittleEndian` - Used by Substrate as part of the SCALE encoding. The amount is LittleEndian encoded
 * - `Float` - Floats are the human-readable format of amounts. This is very user facing.
 * - `Integer` - In combination with decimals, used to represent amounts. These are represented as TS number or BN object
 *
 * This class enables the conversion between these different types. The gateway class heavily relies on this class, passing parameters like decimals and value types out of the box.
 */

export class AmountConverter {
  value: BN;
  decimals: BN;
  valueTypeSize: number;

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
    value?: string | number | BN;
    decimals?: BN | number;
    valueTypeSize?: number;
  }) {
    // Set defaults
    if (!args.decimals) args.decimals = new BN(12);  // default: 12 decimals
    if (!args.valueTypeSize) args.valueTypeSize = 16;

    this.decimals = new BN(args.decimals);
    this.valueTypeSize = args.valueTypeSize;

    if (args.value) {
      // interpret string values correctly
      if (isString(args.value)) {
        if (isHex(args.value)) {
          this.value = fromLeEncoding(args.value as string);
        } else {
          this.value = new BN.BN(args.value, 10);
        }
      } else if (isNumber(args.value)) {
        if (Math.floor(args.value as number) !== args.value) {
          throw new Error(
            "AmountConverter: Float values not supported! Please pass an integer value or convert to it."
          );
        } else {
          this.value = new BN.BN(args.value, 10);
        }
      } else {
        this.value = args.value as BN;
      }
    }
  }

  /**
   * Convert the initialzed value into a float
   */
  toFloat(): number {
    return bnToFloat(this.toBn(), this.decimals);
  }

  /**
   * Convert the initialized value into BN
   */
  toBn(): BN {
    return this.value as BN;
  }

  /**
   * Convert a float parameter into BN, using the set decimals
   * @param value - The value to convert
   */
  floatToBn(value: number): BN {
    const result = floatToBn(value, this.decimals);
    if (this.checkSafeConversion(result)) {
      return result
    } else {
      throw new Error(
        "AmountConverter: Conversion from `number` to `BN` failed -> New integer is larger than its valueTypeSize."
      );
    }
  }

  /**
   * Convert the initialized value into a LittleEndian byte array
   */
  toLeArray(): number[] {
    return toLeEncoding(this.value as BN, this.valueTypeSize);
  }

  /**
   * Convert the initialized value into LittleEndian encoded hex string
   */
  toLeHex(): string {
    return u8aToHex(new Uint8Array(this.toLeArray()));
  }

  /**
   * Check that a conversion from float to integer was done 
   * safely, i.e., the converted value is not larger than 2^valueTypeSize.
   * 
   * @param convertedValue Value to be checked 
   * @returns If the conversion was done safely
   */
  checkSafeConversion(convertedValue: BN): boolean {
    const maxValueForType = new BN.BN(Math.pow(2, this.valueTypeSize * 8))
    if (convertedValue < maxValueForType) {
      return false
    } else {
      return true
    }
  }

  /**
   * Converts a price from integer (`BN` value) to USD (`number` value)
   * 
   * @param price The amount to be converted, with the correct amount of decimals
   * @returns The price in USD
   */
  computeUsdPrice(price: Price): number {
    // Multiply `BN` numbers to keep precision
    const baseNumber = this.value.mul(price.amount)
    // Get the number of decimal places we need to divide by, so we keep the precision safe
    const decimalPositions = this.decimals.add(price.decimals).sub(USD_DECIMAL_PRECISION)
    // Get the value to divide by so decimals are removed
    const decimalsForSafety = new BN(Math.pow(10, decimalPositions.toNumber()))
    // Divide the `BN` by prev. value to remove the extra decimal places
    const baseNumberSafe = baseNumber.div(decimalsForSafety)
    // Get the value as a `number` with decimals
    return baseNumberSafe.toNumber() / (10 ** USD_DECIMAL_PRECISION.toNumber())
  }
}

/**
 * Converts hex LittleEndian to BN
 * @param hex - The hex string to convert
 */
export const fromLeEncoding = (number: string): BN => {
  return new BN.BN(number.split("0x")[1], 16, "le");
};

/**
 * Encode number to LE array
 * @param number - The number to encode
 * @param valueTypeSize - The size of the value type in bytes
 */
export const toLeEncoding = (
  number: BN,
  valueTypeSize: number | undefined
): number[] => {
  if (valueTypeSize === undefined) {
    valueTypeSize = 16 as number;
  }
  return new BN.BN(number).toArray("le", valueTypeSize);
};

/**
 * Converts decimal number to uint as BigNumber. BN.js takes care of correct rounding here
 * @param number - The number to convert
 * @param decimals - The decimals of the number
 */
export const floatToBn = (number: number, decimals: BN): BN => {
  const bigInteger = new BN.BN(number * Math.pow(10, decimals.toNumber()))
  return bigInteger
};

/**
 * Converts uint to a human readable decimal
 * @param number - The integer to convert
 * @param decimals - The decimals of the new number
 */
export const bnToFloat = (number: BN, decimals: BN): number => {
  const bigFloat = Big(number).div(Math.pow(10, decimals.toNumber()))
  return bigFloat
};

/**
 * Generate the optional insurance construct.
 * As the resulting value is only used on t3rn, we rely on the correct default values here
 * @param insurance - The insurance amount
 */
export const optionalInsurance = (
  insurance: number | BN | string,
  reward: number | BN | string
) => {
  const encodedInsurance = new AmountConverter({
    value: insurance,
  }).toLeArray();
  const encodedReward = new AmountConverter({ value: reward }).toLeArray();
  return u8aToHex(new Uint8Array([...encodedInsurance, ...encodedReward]));
};

/** Constant for accounting how many decimals places we allow for USD values */
export const USD_DECIMAL_PRECISION = new BN(4)

// /** Keep track of how many decimals there are in each service */
// export enum DECIMALS_PRECISION {
//   CoinGecko = 4,
//   USD_DECIMAL_PRECISION = 4
// }

export class Price {
  amount: BN;
  decimals: BN;

  constructor(amount: BN, decimals: BN) {
    // Check that both values are `BN` numbers
    if (!BN.isBN(amount) || !BN.isBN(decimals)) {
      this.decimals = decimals;
      this.amount = amount;
    } else {
      throw new Error("Both `amount` and `decimals` fields in `Price` must be integers of type `BN`.")
    }
  }
}
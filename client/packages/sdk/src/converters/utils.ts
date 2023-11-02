/**
 * Converts a `targetId` i.e `roco` string to Uint8Array (bytes array)
 * If it is a byte array, it is returned as is
 *
 * @param targetId - The target to convert
 *
 * @returns The converted target
 */

export const toU8aId = (targetId: string | number[]): number[] => {
  let res: any

  if (typeof targetId === 'string') {
    res = new TextEncoder().encode(targetId)
  } else {
    res = targetId
  }

  if (res.length !== 4) {
    throw new Error('Invalid Id!')
  }

  return res
}

/**
 * Converts a `targetId` in Uint8Array (bytes array) to string
 * If it is a string, it is returned as is
 *
 * @param targetId - The target to convert
 *
 * @returns The converted target
 */

export const toIdString = (targetId: number[] | string): string => {
  if (typeof targetId === 'string') {
    return targetId
  } else {
    return new TextDecoder().decode(Buffer.from(targetId))
  }
}

/**
 * encodes data for exporting. We export in encoded and human format
 * Encoded: We use for seeding portal rust tests
 * Human: Debugging those tests and viewing data
 *
 * @param data - The data to encode
 * @param transactionType - The transaction type
 * @param submissionHeight - The submission height
 */

export const encodeExport = (
  data: any,
  transactionType: string,
  submissionHeight: string,
) => {
  if (Array.isArray(data)) {
    return data.map((entry) =>
      iterateEncode(entry, transactionType, submissionHeight),
    )
  } else {
    return iterateEncode(data, transactionType, submissionHeight)
  }
}

/**
 * @param data - The data to encode
 * @param transactionType - The transaction type
 * @param submissionHeight - The submission height
 *
 * @returns The encoded data
 */

const iterateEncode = (
  data: any,
  transactionType: string,
  submissionHeight: string,
) => {
  let keys = Object.keys(data)
  let result = {}

  if (keys.includes('initialU8aLength')) {
    // this is a polkadot/apiPromise object
    return {
      data: data.toHuman(),
      transaction_type: transactionType,
      encoded_data: data.toHex().substring(2),
    }
  } else {
    for (let i = 0; i < keys.length; i++) {
      result['encoded_' + toSnakeCase(keys[i])] = data[keys[i]]
        .toHex()
        .substring(2)
      result[toSnakeCase(keys[i])] = data[keys[i]].toHuman()
    }

    result['transaction_type'] = transactionType
    result['submission_height'] = submissionHeight

    return result
  }
}

/**
 * Converts a string to snake case
 *
 * @param str - The string to convert
 *
 * @returns The converted string
 *
 * @example
 * toSnakeCase("helloWorld") // hello_world
 * toSnakeCase("hello_world") // hello_world
 * toSnakeCase("helloWorld") // hello_world
 * toSnakeCase("helloWorld") // hello_world
 */

const toSnakeCase = (str: string) =>
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    .map((x) => x.toLowerCase())
    .join('_')

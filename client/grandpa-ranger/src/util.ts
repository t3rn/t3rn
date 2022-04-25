import { tmpdir } from 'os'
import { join } from 'path'
import { writeFile } from 'fs/promises'
import { promisify } from 'util'
import { exec as _exec } from 'child_process'
import { TypeRegistry, createType } from '@polkadot/types';
const registry = new TypeRegistry();

const type = {type: 'Block::Header'}
let data = "0xe90255cc67090a940df176ccea510ecacd85a55fd71acfaf2f1a4f409e1933d019ba4d0dc584d8d3ca18d30c9426f96f44f56cc829f16bf326b06880ed1231278a03fdb93ab69fd3c83b7cebb8f74eeebf5dbd9620ef6c2d4624212e275718303c7183840806617572612009f72e0800000000056175726101013894a477f30424aea70cc40bd246ab486f6b80bc03560dd8d6f0dc7c4c87f048d569be8958cdd4ac0d62489458891b88c3309debc71b38b40bdfd1fa3927c485";

export const decodeCustomType = (type: string, data: string) => {
  const typeObject = { type };
  registry.register(typeObject);
  const res = createType(registry, typeObject.type, data.trim())

  console.log(res)
  return res;
}

export const exec = promisify(_exec)

export function formatEvents(
  events: { event: { section: string; method: string; data: any } }[]
): string[] {
  return events.map(
    ({ event: { data, method, section } }) =>
      `${section}.${method} ${data.toString()}`
  )
}

export async function grandpaDecode(justification: any) {
  const tmpFile = join(tmpdir(), justification.toString().slice(0, 10))

  await writeFile(tmpFile, justification.toString())

  return exec(
    './justification-decoder/target/release/justification-decoder ' + tmpFile
  ).then(cmd => JSON.parse(cmd.stdout))
}

export function decodeHeaderNumber(data: string) {
  // removes the Vec Decoding, bit hacky
  if(data.slice(0, 6) === "0xe902") {
    data = "0x" + data.split("e902")[1];
  }

  const typeObject = { type: 'Block::Header' }
  registry.register(typeObject);
  const res: any = createType(registry, typeObject.type, data)
  return res.number.toNumber();
}
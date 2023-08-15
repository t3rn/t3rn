import { createCircuitContext } from "./utils/circuit.ts"

export type Args<T extends string> = Readonly<
  Partial<{
    [x in T]: string
  }>
>

export type Circuit = Awaited<
  ReturnType<typeof createCircuitContext>
>["circuit"]

export type WithRequiredProperty<Type, Key extends keyof Type> = Type & {
  [Property in Key]-?: Type[Property]
}

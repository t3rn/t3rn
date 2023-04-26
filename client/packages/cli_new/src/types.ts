export type Args<T extends string> = Readonly<
  Partial<{
    [x in T]: string
  }>
>

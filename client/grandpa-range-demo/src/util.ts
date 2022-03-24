export function formatEvents(
  events: { event: { section: string; method: string; data: any } }[]
): string[] {
  return events.map(
    ({ event: { data, method, section } }) =>
      `${section}.${method} ${data.toString()}`
  )
}

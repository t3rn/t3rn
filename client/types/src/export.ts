import * as definitions from "@t3rn/types";

const types = Object.values(definitions).reduce((res, { types }): object => ({ ...res, ...types }), {});

console.log(JSON.stringify(types))
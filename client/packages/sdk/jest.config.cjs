/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  preset: "ts-jest",
  testEnvironment: "node",
  transform: {
    "^.+\\.ts$": "babel-jest",
  },
  globals: {
    "ts-jest": {
      diagnostics: false,
      isolatedModules: true,
      esModuleInterop: true,
    },
  },
};

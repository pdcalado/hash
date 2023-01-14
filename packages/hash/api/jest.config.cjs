/** @type {import('jest').Config} */
module.exports = {
  collectCoverage: process.env.TEST_COVERAGE === "true",
  collectCoverageFrom: [
    "**/*.{c,m,}{j,t}s{x,}",
    "!**/node_modules/**",
    "!**/dist/**",
    "!**/src/collab/**",
  ],
  coverageReporters: ["lcov", "text"],
  moduleNameMapper: {
    "@local/hash-shared(.*)":
      "<rootDir>/../../../libs/@local/hash-shared/src$1",
  },
  preset: "ts-jest",
  testEnvironment: "node",
};

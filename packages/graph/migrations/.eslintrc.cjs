/** @type {import("eslint").Linter.Config} */
module.exports = {
  ...require("@local/eslint-config/generate-workspace-config.cjs")(__dirname),
  env: {
    node: true,
  },
  overrides: [
    {
      // Autogenerated files
      files: ["**/migration/*.ts"],
      rules: {
        "simple-import-sort/exports": "off",
        "simple-import-sort/imports": "off",
        "unicorn/filename-case": "off",
        "unicorn/prefer-node-protocol": "off",
      },
    },
  ],
};

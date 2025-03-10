import { CodegenConfig } from "@graphql-codegen/cli";
import { scalars } from "@hashintel/hash-shared/graphql/scalar-mapping";

const config: CodegenConfig = {
  overwrite: true,
  schema: "./src/graphql/type-defs/**/*.ts",
  require: ["ts-node/register"],
  generates: {
    "./src/graphql/graphql-schema.gen.json": {
      plugins: ["introspection"],
      config: {
        noSchemaStitching: true,
      },
    },
    "./src/graphql/api-types.gen.ts": {
      plugins: ["typescript", "typescript-resolvers", "typescript-operations"],
      documents: ["../shared/src/queries/**/*.ts"],
      hooks: {
        afterOneFileWrite: ["prettier --write"],
      },
      config: {
        avoidOptionals: {
          defaultValue: true,
        },
        noSchemaStitching: true,
        skipTypename: true,
        // Use shared scalars
        scalars,
      },
    },
  },
};

export default config;

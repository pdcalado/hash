FROM node:16.18.1-alpine AS builder

WORKDIR /app

# Required dependencies for compiling the re2 NPM package
RUN apk add --no-cache libc6-compat python3 make g++

# Ensure that the node module layer can be cached
COPY turbo.json .
COPY package.json .
COPY yarn.lock .
RUN yarn install --frozen-lockfile --prefer-offline --force --build-from-source


# Also ensure that frontend node modules can be cached
COPY libs/@local/tsconfig/package.json libs/@local/tsconfig/
COPY libs/@local/eslint-config/package.json libs/@local/eslint-config/
COPY packages/hash/frontend/package.json packages/hash/frontend/
COPY packages/hash/design-system/package.json packages/hash/design-system/
COPY packages/hash/shared/package.json packages/hash/shared/
COPY packages/hash/subgraph/package.json packages/hash/subgraph/
COPY packages/graph/clients/typescript/package.json packages/graph/clients/typescript/
RUN yarn workspace @hashintel/hash-frontend install --frozen-lockfile --prefer-offline

COPY packages/hash/api/codegen.config.ts packages/hash/api/codegen.config.ts
COPY packages/hash/api/src/graphql/type-defs packages/hash/api/src/graphql/type-defs
COPY packages/hash/frontend/codegen.config.ts packages/hash/frontend/codegen.config.ts
COPY packages/hash/frontend/src/graphql/queries packages/hash/frontend/src/graphql/queries
COPY packages/hash/shared/src/queries packages/hash/shared/src/queries
COPY packages/hash/shared/src/graphql packages/hash/shared/src/graphql

RUN yarn workspace @hashintel/hash-frontend codegen

COPY packages/hash/shared/codegen.config.ts packages/hash/shared/codegen.config.ts

RUN yarn workspace @hashintel/hash-shared codegen

COPY libs/@local/tsconfig libs/@local/tsconfig
COPY libs/@local/eslint-config libs/@local/eslint-config
COPY packages/graph/clients/typescript packages/graph/clients/typescript
COPY packages/hash/frontend packages/hash/frontend
COPY packages/hash/design-system packages/hash/design-system
COPY packages/hash/shared packages/hash/shared
COPY packages/hash/subgraph packages/hash/subgraph

WORKDIR /app/packages/hash/frontend
ENV NODE_ENV production
ARG API_ORIGIN
ENV API_ORIGIN ${API_ORIGIN}
ARG FRONTEND_URL
ENV FRONTEND_URL ${FRONTEND_URL}
ARG SYSTEM_ACCOUNT_NAME
ENV SYSTEM_ACCOUNT_NAME ${SYSTEM_ACCOUNT_NAME}
ARG SYSTEM_USER_SHORTNAME
ENV SYSTEM_USER_SHORTNAME ${SYSTEM_USER_SHORTNAME}
RUN yarn next build


#########################################################################################

FROM node:16.18.1-alpine

# Run as a non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
COPY --from=builder --chown=appgroup:appuser /app /app

# Nextjs needs permissions to create caches
RUN chown -R appuser:appgroup /app/packages/hash/frontend/.next
WORKDIR /app

USER appuser
ENV NODE_ENV production

CMD ["yarn", "workspace","@hashintel/hash-frontend", "start"]


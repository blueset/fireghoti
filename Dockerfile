## Install dev and compilation dependencies, build files
FROM docker.io/node:20-alpine as build
WORKDIR /firefish

# Copy only backend-rs pnpm-related files first, to cache efficiently
COPY package.json pnpm-workspace.yaml ./
COPY packages/backend-rs/package.json packages/backend-rs/package.json
COPY packages/backend-rs/npm/linux-x64-musl/package.json packages/backend-rs/npm/linux-x64-musl/package.json
COPY packages/backend-rs/npm/linux-arm64-musl/package.json packages/backend-rs/npm/linux-arm64-musl/package.json

# Install compilation dependencies
RUN apk update && apk add --no-cache build-base linux-headers curl ca-certificates python3 perl
RUN curl --proto '=https' --tlsv1.2 --silent --show-error --fail https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy only backend-rs dependency-related files first, to cache efficiently
COPY packages/macro-rs packages/macro-rs/
COPY packages/backend-rs/src/lib.rs packages/backend-rs/src/
COPY packages/backend-rs/Cargo.toml packages/backend-rs/Cargo.toml
COPY Cargo.toml Cargo.lock ./

# Configure pnpm, and install backend-rs dependencies
RUN corepack enable && corepack prepare pnpm@latest --activate && pnpm --filter backend-rs install
RUN cargo fetch --locked --manifest-path Cargo.toml

# Copy in the rest of the rust files
COPY packages/backend-rs packages/backend-rs/

# Compile backend-rs
RUN NODE_ENV='production' pnpm run --filter backend-rs build

# Copy/Overwrite index.js to mitigate the bug in napi-rs codegen
COPY packages/backend-rs/index.js packages/backend-rs/built/index.js

# Copy only the dependency-related files first, to cache efficiently
COPY packages/backend/package.json packages/backend/package.json
COPY packages/client/package.json packages/client/package.json
COPY packages/sw/package.json packages/sw/package.json
COPY packages/firefish-js/package.json packages/firefish-js/package.json
COPY pnpm-lock.yaml ./

# Install dev mode dependencies for compilation
RUN pnpm install --frozen-lockfile

# Copy in the rest of the files to build
COPY . ./

# Build other workspaces
RUN NODE_ENV='production' pnpm run --recursive --filter '!backend-rs' build && pnpm run build:assets

# Trim down the dependencies to only those for production
RUN find . -path '*/node_modules/*' -delete && pnpm install --prod --frozen-lockfile

## Runtime container
FROM docker.io/node:20-alpine
WORKDIR /firefish

# Install runtime dependencies
RUN apk update && apk add --no-cache zip unzip tini ffmpeg curl

COPY . ./

# Copy node modules
COPY --from=build /firefish/node_modules /firefish/node_modules
COPY --from=build /firefish/packages/backend/node_modules /firefish/packages/backend/node_modules
# COPY --from=build /firefish/packages/sw/node_modules /firefish/packages/sw/node_modules
# COPY --from=build /firefish/packages/client/node_modules /firefish/packages/client/node_modules
COPY --from=build /firefish/packages/firefish-js/node_modules /firefish/packages/firefish-js/node_modules

# Copy the finished compiled files
COPY --from=build /firefish/built /firefish/built
COPY --from=build /firefish/packages/backend/built /firefish/packages/backend/built
COPY --from=build /firefish/packages/backend/assets/instance.css /firefish/packages/backend/assets/instance.css
COPY --from=build /firefish/packages/backend-rs/built /firefish/packages/backend-rs/built
COPY --from=build /firefish/packages/firefish-js/built /firefish/packages/firefish-js/built

RUN corepack enable && corepack prepare pnpm@latest --activate
ENV NODE_ENV=production
VOLUME "/firefish/files"
ENTRYPOINT [ "/sbin/tini", "--" ]
CMD [ "pnpm", "run", "start:container" ]

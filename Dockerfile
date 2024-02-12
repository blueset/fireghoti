## Install dev and compilation dependencies, build files
FROM docker.io/node:20-slim as build
WORKDIR /firefish

# Install compilation dependencies
RUN apt-get update && DEBIAN_FRONTEND='noninteractive' apt-get install -y --no-install-recommends curl build-essential ca-certificates python3
RUN curl --proto '=https' --tlsv1.2 --silent --show-error --fail https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy only the cargo dependency-related files first, to cache efficiently
COPY packages/backend-rs/Cargo.toml packages/backend-rs/Cargo.toml
COPY packages/backend-rs/Cargo.lock packages/backend-rs/Cargo.lock
COPY packages/backend-rs/src/lib.rs packages/backend-rs/src/

# Install cargo dependencies
RUN cargo fetch --locked --manifest-path /firefish/packages/backend-rs/Cargo.toml

# Copy only the dependency-related files first, to cache efficiently
COPY package.json pnpm*.yaml ./
COPY packages/backend/package.json packages/backend/package.json
COPY packages/client/package.json packages/client/package.json
COPY packages/sw/package.json packages/sw/package.json
COPY packages/firefish-js/package.json packages/firefish-js/package.json
COPY packages/megalodon/package.json packages/megalodon/package.json
COPY packages/backend-rs/package.json packages/backend-rs/package.json
COPY packages/backend-rs/npm/linux-x64-musl/package.json packages/backend-rs/npm/linux-x64-musl/package.json
COPY packages/backend-rs/npm/linux-arm64-musl/package.json packages/backend-rs/npm/linux-arm64-musl/package.json

# Configure pnpm, and install dev mode dependencies for compilation
RUN corepack enable && corepack prepare pnpm@latest --activate && pnpm i --frozen-lockfile

# Copy in the rest of the rust files
COPY packages/backend-rs packages/backend-rs/

# Compile backend-rs
RUN pnpm run --filter backend-rs build

# Copy in the rest of the files to compile
COPY . ./
RUN env NODE_ENV=production sh -c "pnpm run --filter '!backend-rs' build && pnpm run gulp"

# Trim down the dependencies to only those for production
RUN pnpm i --prod --frozen-lockfile

## Runtime container
FROM docker.io/node:20-slim
WORKDIR /firefish

# Install runtime dependencies
RUN apt-get update && DEBIAN_FRONTEND='noninteractive' apt-get install -y --no-install-recommends zip unzip tini ffmpeg

COPY . ./

COPY --from=build /firefish/packages/megalodon /firefish/packages/megalodon

# Copy node modules
COPY --from=build /firefish/node_modules /firefish/node_modules
COPY --from=build /firefish/packages/backend/node_modules /firefish/packages/backend/node_modules
COPY --from=build /firefish/packages/sw/node_modules /firefish/packages/sw/node_modules
COPY --from=build /firefish/packages/client/node_modules /firefish/packages/client/node_modules
COPY --from=build /firefish/packages/firefish-js/node_modules /firefish/packages/firefish-js/node_modules

# Copy the finished compiled files
COPY --from=build /firefish/built /firefish/built
COPY --from=build /firefish/packages/backend/built /firefish/packages/backend/built
COPY --from=build /firefish/packages/backend/assets/instance.css /firefish/packages/backend/assets/instance.css
COPY --from=build /firefish/packages/backend-rs/built /firefish/packages/backend-rs/built

RUN corepack enable && corepack prepare pnpm@latest --activate
ENV NODE_ENV=production
VOLUME "/firefish/files"
ENTRYPOINT [ "/usr/bin/tini", "--" ]
CMD [ "pnpm", "run", "migrateandstart" ]

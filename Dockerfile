## Install dev and compilation dependencies, build files
FROM docker.io/node:20-alpine as build
WORKDIR /firefish

# Install compilation dependencies
RUN apk update && apk add --no-cache build-base linux-headers curl ca-certificates python3 perl
RUN curl --proto '=https' --tlsv1.2 --silent --show-error --fail https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy only the cargo dependency-related files first, to cache efficiently
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY packages/backend-rs/Cargo.toml packages/backend-rs/Cargo.toml
COPY packages/backend-rs/src/lib.rs packages/backend-rs/src/
COPY packages/macro-rs/Cargo.toml packages/macro-rs/Cargo.toml
COPY packages/macro-rs/src/lib.rs packages/macro-rs/src/

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
RUN corepack enable && corepack prepare pnpm@latest --activate && pnpm install --frozen-lockfile

# Copy in the rest of the rust files
COPY packages/backend-rs packages/backend-rs/
# COPY packages/macro-rs packages/macro-rs/

# Compile backend-rs
RUN NODE_ENV='production' pnpm run --filter backend-rs build

# Copy/Overwrite index.js to mitigate the bug in napi-rs codegen
COPY packages/backend-rs/index.js packages/backend-rs/built/index.js

# Copy in the rest of the files to compile
COPY . ./
RUN NODE_ENV='production' pnpm run --filter firefish-js build
RUN NODE_ENV='production' pnpm run --recursive --parallel --filter '!backend-rs' --filter '!firefish-js' build && pnpm run gulp

# Trim down the dependencies to only those for production
RUN find . -path '*/node_modules/*' -delete && pnpm install --prod --frozen-lockfile

## Runtime container
FROM docker.io/node:20-alpine
WORKDIR /firefish

# Install runtime dependencies
RUN apk update && apk add --no-cache zip unzip tini ffmpeg curl

COPY . ./

COPY --from=build /firefish/packages/megalodon /firefish/packages/megalodon

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

RUN corepack enable && corepack prepare pnpm@latest --activate
ENV NODE_ENV=production
VOLUME "/firefish/files"
ENTRYPOINT [ "/sbin/tini", "--" ]
CMD [ "pnpm", "run", "start:container" ]

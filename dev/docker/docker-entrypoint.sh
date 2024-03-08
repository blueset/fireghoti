#!/bin/sh

set -xeu
node --version

# Check Environment Initialized Flag
if [ ! -f '/.firefish_env_initialized' ]; then

	# Install compilation dependencies
	apk update
	apk add --no-cache build-base linux-headers curl ca-certificates python3 git postgresql16-client zip unzip ffmpeg
	curl -vvv --proto '=https' --tlsv1.2 --show-error --fail https://sh.rustup.rs | sh -s -- -y

	# Add Cargo PATH
	PATH="/root/.cargo/bin:${PATH}"

	# If Firefish not exist
	if [ ! -f '/firefish/README.md' ]; then

		# Clone Firefish
		cd /
		git clone -v https://firefish.dev/firefish/firefish.git

		# Configuring a new server
		cd /firefish
		cp .config/devenv.yml .config/default.yml

		URL="$(echo "${URL}" | sed 's#/#\\/#g')"
		sed -i'.bak' "s/http:\/\/localhost:3030/${URL}/" .config/default.yml

	fi

	# Configure postgres, add pgroonga search
	psql --user=firefish --host=firefish_db --dbname=firefish_db --command='CREATE EXTENSION IF NOT EXISTS pgroonga;'

	# Configure pnpm, and install dev mode dependencies for compilation
	cd /firefish
	corepack enable
	corepack prepare pnpm@latest --activate
	pnpm install --prod false

fi

# Add Environment Initialized Flag
touch /.firefish_env_initialized

# Add Cargo PATH
PATH="/root/.cargo/bin:${PATH}"

# Start a new server
cd /firefish
pnpm install --prod false
pnpm run build:debug
pnpm run migrate
pnpm run start

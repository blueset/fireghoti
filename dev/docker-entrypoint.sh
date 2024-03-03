#!/bin/sh

set -xeu
node --version

# Check Environment Initialized Flag
if [ ! -f '/.firefish_env_initialized' ]; then

	# Install compilation dependencies
	apt-get update
	DEBIAN_FRONTEND='noninteractive' apt-get install -y --no-install-recommends curl build-essential ca-certificates python3 postgresql-client-15
	curl -vvv --proto '=https' --tlsv1.2 --show-error --fail https://sh.rustup.rs | sh -s -- -y

	# Add Cargo PATH
	PATH="/root/.cargo/bin:${PATH}"

	if [ ! -f '/firefish/README.md' ]; then

		# Download Firefish and decompress
		curl -vvv -O --proto '=https' --tlsv1.2 --show-error --fail https://firefish.dev/firefish/firefish/-/archive/develop/firefish-develop.tar.bz2
		tar -xjvf firefish-develop.tar.bz2 --strip-components 1 -C /firefish

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

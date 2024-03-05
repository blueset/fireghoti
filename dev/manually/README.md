# Manually Make Development Environment

For scenarios with special needs, you may want to manually create a development environment. Hope this article can help you.

## Introduction

This article is written based on the Debian Bookworm. Other systems can refer to this article for deployment. However, it is recommended that new users use the same system or Docker environment as ours to avoid wasting time on environment configuration issues.

The versions of Node.js, Rust, PostgreSQL that come with Debian Bookworm are low, the latest official versions of these components are used to install them. Other components are installed using the apt package manager that comes with the system.

## Allow `sudo` command

```sh
su -
apt install -y -V sudo
# user is your username
usermod -aG sudo user
reboot
```

## Install Base Requirements

```sh
sudo apt update
sudo apt install -y -V wget curl git ca-certificates
```

## Install Node.js

The latest version at the time of writing is v21.6.2. Please replace it with the latest Node.js version number during installation. Details can be found in [nodejs.org](https://nodejs.org) .

1. Download and extract.

```sh
VERSION=v21.6.2
DISTRO=linux-x64
sudo mkdir -p /usr/local/lib/nodejs
wget https://nodejs.org/dist/v21.6.2/node-$VERSION-$DISTRO.tar.xz
sudo tar -xJvf node-$VERSION-$DISTRO.tar.xz -C /usr/local/lib/nodejs
```

2. Open your `.profile` and `/root/.profile` files.

```sh
nano ~/.profile
sudo nano /root/.profile
```

3. Add below content at below of this two file to set the environment variable.

```sh
# Nodejs
VERSION=v21.6.2
DISTRO=linux-x64
export PATH=/usr/local/lib/nodejs/node-$VERSION-$DISTRO/bin:$PATH
```

4. Refresh `PATH` and test.

```sh
. ~/.profile
node -v
# Switching to root
sudo -i
. ~/.profile
node -v
exit
```

## Install Rust

1. Running this script and choose "Proceed with installation" option.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. ~/.profile
cargo -V
```

## Install PostgreSQL with PGroonga extension

```sh
wget https://apache.jfrog.io/artifactory/arrow/$(lsb_release --id --short | tr 'A-Z' 'a-z')/apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb
sudo apt install -y -V ./apache-arrow-apt-source-latest-$(lsb_release --codename --short).deb
wget https://packages.groonga.org/debian/groonga-apt-source-latest-$(lsb_release --codename --short).deb
sudo apt install -y -V ./groonga-apt-source-latest-$(lsb_release --codename --short).deb
echo "deb http://apt.postgresql.org/pub/repos/apt/ $(lsb_release --codename --short)-pgdg main" | sudo tee /etc/apt/sources.list.d/pgdg.list
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
sudo apt update
sudo apt install -y -V postgresql-16-pgdg-pgroonga
```

## Configuration PostgreSQL

Create Firefish database, user and PGroonga extension.

```sh
sudo --user=postgres createuser --no-createdb --no-createrole --no-superuser --encrypted --pwprompt firefish
sudo --user=postgres createdb --encoding='UTF8' --owner=firefish firefish_db
sudo --user=postgres psql --command='CREATE EXTENSION pgroonga;' --dbname=firefish_db
```

## Install Redis, Python 3 and build-essential

```sh
sudo apt update
sudo apt install -y -V redis python3 build-essential
```

## Download and configuration Firefish

1. Download Firefish and Copy example configuration file.

```sh
# cd /path/to/your/firefish
git clone https://firefish.dev/firefish/firefish.git
cd firefish/
cp .config/devenv.yml .config/default.yml
sed -i "s/host: firefish_db/host: localhost/" .config/default.yml
sed -i "s/host: firefish_redis/host: localhost/" .config/default.yml
```

2. Open your `default.yml` files and make changes like `URL`, `db/host` `redis/host`.

```sh
nano .config/default.yml
```

## Install package

1. Let corepack enable.

```sh
# Switching to root
sudo -i
# cd /path/to/your/firefish
cd /home/user/firefish
npm i -g pm2
corepack enable
exit
```

2. Install dependency.

```sh
corepack prepare pnpm@latest --activate
pm2 install pm2-logrotate
pnpm install --frozen-lockfile --prod false
```

> **Note**
> [`pm2-logrotate`](https://github.com/keymetrics/pm2-logrotate/blob/master/README.md) ensures that log files don't infinitely gather size, as Firefish produces a lot of logs.

## Start

1. Build and migrate

```sh
pnpm install --prod false
NODE_ENV=production
pnpm run build:debug
pnpm run migrate
```

2. Start Firefish

```sh
pnpm run start
```

2. Wait until the following message shows up.

```log
DONE *  [core boot]     All workers started
DONE *  [core boot]     Now listening on port 3030 on https://your_firefish_url.example.com
```

3. A fresh Firefish environment is created on the URL you have set!

4. If you want Firefish to run in the background, start it with this command.

```sh
pm2 start "NODE_ENV=production pnpm run start" --name Firefish
# When you want display log
pm2 logs Firefish
```

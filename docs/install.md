# Install Production Environment

Hope this article can help you about install production environment.

## Introduction

This article is written based on the Debian Bookworm. Other systems can refer to this article for deployment. However, it is recommended that new users use the same system or Docker environment as ours to avoid wasting time on environment configuration issues.

The versions of Node.js, Rust, PostgreSQL, DragonflyDB that come with Debian Bookworm are low or not have, the latest official versions of these components are used to install them. Other components are installed using the apt package manager that comes with the system.

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

1. Execute this to running `psql` as `postgres` user.

```sh
sudo -u postgres psql
```

2. Create Firefish database, user and PGroonga extension. **Please change the password.**

```sql
CREATE DATABASE firefish WITH ENCODING = 'UTF8';
\connect firefish
CREATE EXTENSION IF NOT EXISTS pgroonga;
CREATE USER firefish WITH PASSWORD 'password';
ALTER USER firefish WITH SUPERUSER;
GRANT ALL ON DATABASE firefish TO firefish;
```

3. Run `exit` to return.

## Install DragonflyDB (Cache)

```sh
wget https://dragonflydb.gateway.scarf.sh/latest/dragonfly_amd64.deb
sudo dpkg -i dragonfly_amd64.deb
```

## Configuration DragonflyDB

If you experience a lot of traffic, it's a good idea to set up another Redis-compatible caching server. If you don't set one one up, it'll fall back to the mandatory Redis server. DragonflyDB is the recommended option due to its unrivaled performance and ease of use.

1. Open your `dragonfly.conf` files.

```sh
sudo nano /etc/dragonfly/dragonfly.conf
```

2. Add content at below of this file to set the different port variable because default port is 6379.

```conf
--port=6380
```

3. Run `sudo systemctl restart dragonfly` to restart it.

## Install Caddy, Redis, Python 3 and build-essential

If you already have experience using nginx, you can consider replacing caddy with nginx here.

```sh
sudo apt update
sudo apt install -y -V caddy redis python3 build-essential
```

## Configuration Caddy

If you replaced nginx in the previous step, please refer to the "Other reverse proxy server" chapter in the document for configuration.

1. Run this to modify caddy configuration.

```sh
sudo nano /etc/caddy/Caddyfile
```

2. Add this below, should change `example.tld` to your domain.

```conf
example.tld {
    reverse_proxy http://127.0.0.1:3000
}
```

3. Running `sudo systemctl restart caddy` to apply.

## Download and configuration Firefish

1. Download Firefish and Copy example configuration file.

```sh
# cd /path/to/your/firefish
git clone https://firefish.dev/firefish/firefish.git
cd firefish/
git checkout main
cp .config/example.yml .config/default.yml
```

> **Note**
> By default, you're on the develop branch. Run `git checkout main` to switch to the Main branch.

2. Open your `default.yml` files and make changes like `URL`, `db` and `reservedUsernames`.

```sh
nano .config/default.yml
```

## Install Firefish

**run these steps to update Firefish in the future!**

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
pnpm install --frozen-lockfile --prod false
pm2 install pm2-logrotate
```

> **Note**
> [`pm2-logrotate`](https://github.com/keymetrics/pm2-logrotate/blob/master/README.md) ensures that log files don't infinitely gather size, as Firefish produces a lot of logs.

3. Build and migrate

```sh
pnpm install --prod false
NODE_ENV=production pnpm run build && pnpm run migrate
```

4. Start Firefish

```sh
pm2 start "NODE_ENV=production pnpm run start" --name Firefish
pm2 logs Firefish
```

5. Wait until the following message shows up.

```log
1|Firefish | DONE *     [core boot]     All workers started
1|Firefish | DONE *     [core boot]     Now listening on port 3000 on https://your_firefish_url.example.com (default value: https://localhost:3000)
```

6. A fresh Firefish environment is created on the URL you have set!

7. By the way, Please use content at below to generate vapid keys to enable Push-Notifications.

```sh
# Switching to root
sudo -i
npm install -g web-push
web-push generate-vapid-keys
exit
```

## Customize

- To add custom CSS for all users, edit `./custom/assets/instance.css`.
- To add static assets (such as images for the splash screen), place them in the `./custom/assets/` directory. They'll then be available on `https://yourserver.tld/static-assets/filename.ext`.
- To add custom locales, place them in the `./custom/locales/` directory. If you name your custom locale the same as an existing locale, it will overwrite it. If you give it a unique name, it will be added to the list. Also make sure that the first part of the filename matches the locale you're basing it on. (Example: `en-FOO.yml`)
- To add custom error images, place them in the `./custom/assets/badges` directory, replacing the files already there.
- To add custom sounds, place only mp3 files in the `./custom/assets/sounds` directory.
- To update custom assets without rebuilding, just run `pnpm run gulp`.
- To block ChatGPT, CommonCrawl, or other crawlers from indexing your instance, uncomment the respective rules in `./custom/robots.txt`.

## Other reverse proxy server

### Nginx

- Run `sudo cp ./firefish.nginx.conf /etc/nginx/sites-available/ && cd /etc/nginx/sites-available/` .
- Edit `firefish.nginx.conf` to reflect your server properly.
- Run `sudo ln -s ./firefish.nginx.conf ../sites-enabled/firefish.nginx.conf` .
- Run `sudo nginx -t` to validate that the config is valid, then restart the NGINX service.

## Tips & Tricks

- When editing the config file, please don't fill out the settings at the bottom. They're designed *only* for managed hosting, not self hosting. Those settings are much better off being set in Firefish's control panel.
- Port 3000 (used in the default config) might be already used on your server for something else. To find an open port for Firefish, run `for p in {3000..4000}; do ss -tlnH | tr -s ' ' | cut -d" " -sf4 | grep -q "${p}$" || echo "${p}"; done | head -n 1`. Replace 3000 with the minimum port and 4000 with the maximum port if you need it.
- I'd recommend you use a S3 Bucket/CDN for Object Storage, especially if you use Docker.
- When using object storage, setting a proper `Access-Control-Allow-Origin` response header is highly recommended.
- I'd ***strongly*** recommend against using CloudFlare, but if you do, make sure to turn code minification off.
- For push notifications, run `npx web-push generate-vapid-keys`, then put the public and private keys into Control Panel > General > ServiceWorker.
- For translations, make a [DeepL](https://deepl.com) account and generate an API key, then put it into Control Panel > General > DeepL Translation.
- To add another admin account:
  - Go to the user's page > 3 Dots > About > Moderation > turn on "Moderator"
  - Go back to Overview > click the clipboard icon next to the ID
  - Run `psql -d firefish` (or whatever the database name is)
  - Run `UPDATE "user" SET "isAdmin" = true WHERE id='999999';` (replace `999999` with the copied ID)
  - Restart your Firefish server

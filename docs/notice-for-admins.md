# Unreleased

The full-text search engine in Firefish has been changed to [PGroonga](https://pgroonga.github.io/). This is no longer an optional feature, so please enable PGroonga on your system. If you are using Sonic, Meilisearch, or Elasticsearch, you can also uninstall it from your system and remove the settings from `.config/default.yml`.

## For systemd/pm2 users

### 1. Install PGroonga

Please execute `psql --version` to check your PostgreSQL major version. This will print a message like this:

```text
psql (PostgreSQL) 16.1
```

In this case, your PostgreSQL major version is `16`.

There are official installation instructions for many operating systems on <https://pgroonga.github.io/install>, so please follow the instructions on this page. However, since many users are using Ubuntu, and there are no instructions for Arch Linux, we explicitly list the instructions for Ubuntu and Arch Linux here. Please keep in mind that this is not official information and the procedures may change.

#### Ubuntu

1. Add apt repository
    ```sh
    sudo apt install -y software-properties-common
    sudo add-apt-repository -y universe
    sudo add-apt-repository -y ppa:groonga/ppa
    sudo apt install -y wget lsb-release
    wget https://packages.groonga.org/ubuntu/groonga-apt-source-latest-$(lsb_release --codename --short).deb
    sudo apt install -y -V ./groonga-apt-source-latest-$(lsb_release --codename --short).deb
    echo "deb http://apt.postgresql.org/pub/repos/apt/ $(lsb_release --codename --short)-pgdg main" | sudo tee /etc/apt/sources.list.d/pgdg.list
    wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
    sudo apt update
    ```
2. Install PGroonga
    ```sh
    # Please replace "16" with your PostgreSQL major version
		sudo apt install postgresql-16-pgdg-pgroonga
    ```

#### Arch Linux

You can install PGroonga from the Arch User Repository.

```sh
git clone https://aur.archlinux.org/pgroonga.git && cd pgroonga && makepkg -si
# or paru -S pgroonga
# or yay -S pgroonga
```

### 2. Enable PGroonga

After the instllation, please execute this command to enable PGroonga:

```sh
sudo --user=postgres psql --dbname=your_database_name --command='CREATE EXTENSION pgroonga;'
```

The database name can be found in `.config/default.yml`.
```yaml
db:
  port: 5432
  db: database_name  # substitute your_database_name with this
  user: firefish
  pass: password
```

## For Docker/Podman users

Please edit your `docker-compose.yml` to replace the database container image from `docker.io/postgres` to `docker.io/groonga/pgroonga`.

Please make sure to use the same PostgreSQL version. If you are using `docker.io/postgres:16-alpine` (PostgreSQL v16), the corresponding image tag is `docker.io/groonga/pgroonga:latest-alpine-16` (or `docker.io/groonga/pgroonga:latest-alpine-16-slim`).

The list of tags can be found on <https://hub.docker.com/r/groonga/pgroonga/tags>.

```yaml
db:
  restart: unless-stopped
  image: docker.io/groonga/pgroonga:latest-alpine-16-slim  # change here
  container_name: firefish_db
```

After that, execute this command to enable PGroonga:

```sh
docker-compose up db --detach && docker-compose exec db sh -c 'psql --user="${POSTGRES_USER}" --dbname="${POSTGRES_DB}" --command="CREATE EXTENSION pgroonga;"'
# or podman-compose up db --detach && podman-compose exec db sh -c 'psql --user="${POSTGRES_USER}" --dbname="${POSTGRES_DB}" --command="CREATE EXTENSION pgroonga;"'
```

Once this is done, you can start Firefish as usual.

```sh
docker pull registry.firefish.dev/firefish/firefish && docker-compose up --detach
# or podman pull registry.firefish.dev/firefish/firefish && podman-compose up --detach
```

# v20240301

## For all users

A new setting item has been added to control the log levels, so please consider updating your `.config/default.yml`. ([example settings](https://firefish.dev/firefish/firefish/-/blob/e7689fb302a0eed192b9515162258a39800f838a/.config/example.yml#L170-179))

# v20240225

## For Docker/Podman users

- The bug where `custom` directory was not working has (finally) been fixed. Please add the `custom` directory to `volumes` in your `docker-compose.yml`:
    ```yaml
    services:
      web:
        image: registry.firefish.dev/firefish/firefish:latest
        # and so on ...

        volumes:
          - ./custom:/firefish/custom:ro  # <- Please add this line
          - ./files:/firefish/files
          - ./.config:/firefish/.config:ro
    ```

# v20240222

## For Docker/Podman users

- You only need to pull the new container image (`docker/podman pull`) to upgrade your server, so we assume that many of you don't update the code (`git pull --ff`), but it's still worth noting here that we have renamed `docker-compose.yml` to `docker-compose.example.yml` in the repository, and `docker-compose.yml` is now set to be untracked by git.
    - Since `docker-compose.yml` may be edited by users (e.g., change port number, add reverse proxy), it shouldn't have been tracked by git in the first place.
    - If you want to update the repository (`git pull --ff`), please take the following steps to keep your `docker-compose.yml`:
        1. Backup (make a copy) your `docker-compose.yml`
            ```sh
            cp docker-compose.yml /tmp/my-docker-compose.yml  # or somewhere else
            ```
        2. Restore the original `docker-compose.yml` so it doesn't conflict with the upstream changes
            ```sh
            git checkout -- docker-compose.yml
            ```
        3. Pull the new code
            ```sh
            git switch main
            git pull --ff
            ```
        4. Bring back your `docker-compose.yml`
            ```sh
            mv /tmp/my-docker-compose.yml docker-compose.yml
            ```
    - If any modifications are needed to `docker-compose.yml` in the future, we will provide a notice.
    - Also, PostgreSQL v12.2 (`docker.io/postgres:12.2-alpine`) has been used in this compose file, but we highly recommend that you upgrade it to a newer version (e.g., `docker.io/postgres:16-alpine`).
        - Note: some manual (painful) operations are needed to upgrade the PostgreSQL major version, so please be careful when performing upgrades: <https://github.com/docker-library/postgres/issues/37>

# v20240214

## For systemd/pm2 users

- Required Rust version has been bumped from v1.70 to v1.74.
    ```sh
    cargo --version  # check version
    rustup update    # update version
    ```

# v20240213

## For systemd/pm2 users

- `packages/backend/native-utils` can be removed.
    - This directory was removed in the repository, but it's not completely removed from your system by `git pull --ff`, because some folders like `packages/backend/native-utils/built` are not tracked by git.

    ```sh
    rm --recursive --force packages/backend/native-utils
    ```

# v20240206

## For all users

- The git repository has been moved, so please update the `git remote` url.
    ```sh
    git remote set-url origin https://firefish.dev/firefish/firefish.git
    ```

## For systemd/pm2 users

- Required Rust version has been bumped from v1.68 to v1.70.
- `libvips` is no longer required (unless your server os is *BSD), so you may uninstall it from your system. Make sure to execute the following commands after that:
    ```sh
    pnpm clean-npm
    pnpm install
    ```

## For Docker/Podman users

- The image tag has been changed to `registry.firefish.dev/firefish/firefish:latest`, so please update `docker-compose.yml`.

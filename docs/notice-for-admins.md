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

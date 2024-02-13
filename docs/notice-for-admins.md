# v20240214

## For systemd/pm2 users

- Required Rust version has been bumped from v1.68 to v1.74.
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

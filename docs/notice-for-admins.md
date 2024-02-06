## For all users

- The git repository has been moved, so please update the `git remote` url.
    ```sh
    git remote set-url origin https://firefish.dev/firefish/firefish.git
    ```

## For systemd users

- `libvips` is no longer required (unless your server os is *BSD), so you may uninstall it from your system. Make sure to execute the following commands after that:
    ```sh
    pnpm clean-npm
    pnpm install
    ```

## For Docker/Podman users

- The image tag has been changed to `registry.firefish.dev/firefish/firefish:latest`, so please update `docker-compose.yml`.

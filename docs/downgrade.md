# Downgrading to `v20240206`

## systemd

1. Stop the Firefish service
    ```sh
    sudo systemctl stop your-firefish-service.service
    ```
1. Take a backup
1. Switch back to the `v20240206` tag
    ```sh
    git switch v20240206
    ```
1. Rebuild Firefish
    ```sh
    pnpm install --frozen-lockfile
    NODE_ENV='production' NODE_OPTIONS='--max_old_space_size=3072' pnpm run rebuild
    ```
1. Start the Firefish service and confirm that Firefish is downgraded
    ```sh
    sudo systemctl start your-firefish-service.service
    ```

## Docker/Podman

1. Stop the container
    ```sh
    docker-compose down
    # or podman-compose down
    ```
1. Take a backup
1. Change the image tag to `registry.firefish.dev/firefish/firefish:v20240206`
    ```sh
    vim docker-compose.yml
    ```

    ```yaml
    version: "3"

    services:
      web:
        image: registry.firefish.dev/firefish/firefish:v20240206  # here
    ```
1. Start the container and confirm that Firefish is downgraded
    ```sh
    docker-compose up --detach
    # or podman-compose up --detach
    ```

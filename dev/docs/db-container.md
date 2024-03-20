# Set up database containers and run Firefish locally

## Prerequisites

- Dependencies
  - git
  - Node.js
  - pnpm
  - Rust toolchain
  - FFmpeg
  - Container runtime
    - [Docker](https://docs.docker.com/get-docker/)
    - [Podman](https://podman.io/docs/installation) and [Podman Compose](https://github.com/containers/podman-compose)
    - [containerd](https://github.com/containerd/containerd) and [nerdctl](https://github.com/containerd/nerdctl)
    - or whatever you want to use
- The following ports are not in use
  - 25432
  - 26379

You can refer to [local-installation.md](./local-installation.md) to install the dependencies.

## Configure the environment

1. Fork the Firefish repository on GitLab
1. Clone your Firefish repository
    ```sh
    git clone https://firefish.dev/your-user-name/firefish.git
    cd firefish
    ```
1. Create `.config/default.yml` with the following content
    ```yaml
    # You can change the port if 3000 is already used
    url: http://localhost:3000
    port: 3000

    db:
      host: localhost
      port: 25432
      db: firefish_db
      user: firefish
      pass: password

    redis:
      host: localhost
      port: 26379

    logLevel: [
      'error',
      'success',
      'warning',
      'info'
    ]
    ```
1. Start database containers
    ```sh
    cd dev/db-container
    docker compose up --detach
    # or podman-compose up --detach
    # or whatever
    
    # go back to the repository root
    cd ../..
    ```

## Build and start Firefish

1. Build Firefish
    ```sh
    pnpm install
    pnpm run build:debug
    ```
1. Execute database migrations
    ```sh
    pnpm run migrate
    ```
1. Start Firefish
    ```sh
    pnpm run start
    ```
    You can access to the local Firefish server on http://localhost:3000 after this message shows up!
    ```
    DONE *  [core boot]     All workers started
    DONE *  [core boot]     Now listening on port 3000 on http://localhost:3000
    ```

## Reset the environment

You can recreate a fresh local Firefish environment by recreating the database containers:

```sh
cd dev/db-container
docker compose down
docker compose up --detach
cd ../..

pnpm run migrate
pnpm run start
```

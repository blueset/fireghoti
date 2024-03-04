# Containerized Environment

The Firefish repo comes with a new containerized environment to help make development!

## Prerequisites

- Latest [Docker](https://docs.docker.com/get-docker/) installation
    - Alternatively, you can use [Podman](https://podman.io/docs/installation) and [Podman Compose](https://github.com/containers/podman-compose).
- The following ports are not in use
    - 3030
    - 25432
    - 26379

## Start up the environment

1. Download the [`dev/docker` directory](https://firefish.dev/firefish/firefish/-/tree/develop/dev/docker) and execute `chmod +x docker-entrypoint.sh`.
    - Alternatively, you can manually run `git clone https://firefish.dev/firefish/firefish.git && cd firefish` to fetch needed files, or let the script take care of itself.
1. Open `docker-compose.yml` and set `URL` to the URL you want to use (or leave it to `http://localhost:3030`).
1. Run `docker compose up`. This will build the environment, dependencies and prepare the needed config files.
    - If you use Podman, you should run `podman-compose up` instead.
1. Wait until the following message shows up.
    ```
    DONE *  [core boot]     All workers started
    DONE *  [core boot]     Now listening on port 3030 on https://your_firefish_url.example.com
    ```
1. A fresh Firefish environment is created on the URL you have set!

When you want to restart the dev server, you just need to terminate the process (a.k.a. press `Ctrl+C`) and run `docker compose up` again.

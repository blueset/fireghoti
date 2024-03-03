# Firefish Developer Docs

## Docker-based Environment

The Firefish repo comes with a new Docker-based environment to help make development as easy as possible!

### Prerequisites

- Installed [Docker](https://docs.docker.com/get-docker/) (use the comman on their website) .
- If your Docker version is older, you may also need to manually install Docker Compose, and in the following instructions
`docker compose` should be replaced with `docker-compose` .
- It is necessary to confirm that port 3000, 25432 and 26379 are not used by other programs or services.

Once Docker is installed to your computer, follow these next few steps to running:

- Download `docker-compose.yml` and `docker-entrypoint.sh` in dev folder to the new folder on your computer.
- If you are not running on your PC, open `docker-compose.yml`, set `URL` and save.
- Run `chmod 777 docker-entrypoint.sh`, for initialization script can be execute.
- Run `docker compose up`, This will build the environment, dependencies and prepare the needed config files.
- Once you see the Firefish banner printed in your screen, means initialization finished.
- Open http://localhost:3000 or `URL` on `docker-compose.yml` in your web browser.
- You should now see the admin user creation screen!

Note: When you want to restart a dev server, all you need to do is press `Ctrl+C` and run `docker compose up`, no other steps are necessary.

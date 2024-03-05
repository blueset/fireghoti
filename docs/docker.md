# Running a Firefish server with Docker

## Prerequisites

- Latest [Docker](https://docs.docker.com/get-docker/) installation
    - Alternatively, you can use [Podman](https://podman.io/docs/installation) and [Podman Compose](https://github.com/containers/podman-compose).
- The following ports are not in use
		- 80
		- 443
    - 3000
    - 5432
    - 6379

## Pre-built docker container

[registry.firefish.dev/firefish/firefish](https://firefish.dev/firefish/firefish/container_registry)

## Config files

There are example config files that you can use to build the container from source

- docker-compose.example.yml (**compose file**)
- .config/docker_example.env (**db config settings**)
- .config/default.yml (**Firefish server settings**)

## Configuring

Copy the files:

```sh
cp docker-compose.example.yml docker-compose.yml
cp .config/example.yml .config/default.yml
cp .config/docker_example.env .config/docker.env
```

then edit them according to your environment.
You can configure `docker.env` with anything you like, but you will have to pay attention to the `default.yml` file:

- `url` should be set to the URL you will be hosting the web interface for the server at.
- `host`, `db`, `user`, `pass` will have to be configured in the `PostgreSQL configuration` section - `host` is the name of the postgres container (eg: *firefish_db_1*), and the others should match your `docker.env`.
- `host`will need to be configured in the *Redis configuration* section - it is the name of the redis container (eg: *firefish_redis_1*)

Everything else can be left as-is.

## Running

The [prebuilt container for firefish](https://firefish.dev/firefish/firefish/container_registry) is fairly large, and may take a few minutes to download and extract using docker.

Copy `docker-compose.yml` and the `config/` to a directory, then run the **docker compose** command: `docker compose up -d` .
    - If you use Podman, you should run `podman-compose up` instead.

NOTE: This will take some time to come fully online, even after download and extracting the container images, and it may emit some error messages before completing successfully. Specifically, the `db` container needs to initialize and so isn't available to the `web` container right away. Only once the `db` container comes online does the `web` container start building and initializing the firefish tables.

Once the server is up you can use a web browser to access the web interface at `http://serverip:3000` (where `serverip` is the IP of the server you are running the firefish server on).

## Install reverse proxy

### Caddy (recommended for new users)

- Add the following block to your `Caddyfile`, replacing `example.tld` with your own domain:

```caddy
example.tld {
    reverse_proxy http://127.0.0.1:3000
}
```

- Reload your caddy configuration

### Nginx (recommended)

- Run `sudo cp ./firefish.nginx.conf /etc/nginx/sites-available/ && cd /etc/nginx/sites-available/`
- Edit `firefish.nginx.conf` to reflect your server properly
- Run `sudo ln -s ./firefish.nginx.conf ../sites-enabled/firefish.nginx.conf`
- Run `sudo nginx -t` to validate that the config is valid, then restart the NGINX service.

### Apache

> **Warning**
> Apache has some known problems with Firefish. Only use it if you have to.

- Run `sudo cp ./firefish.apache.conf /etc/apache2/sites-available/ && cd /etc/apache2/sites-available/`
- Edit `firefish.apache.conf` to reflect your server properly
- Run `sudo a2ensite firefish.apache` to enable the site
- Run `sudo service apache2 restart` to reload apache2 configuration

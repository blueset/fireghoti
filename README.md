<div align="center">
<img src="./title.svg" alt="Firefish logo" style="border-radius:50%" width="400"/>

**🌎 Firefish is an open source, decentralized social media platform that's free forever! 🚀**

</div>

<div>

<img src="./animated.svg" align="right" height="320px"/>

Firefish is based off of Misskey, a powerful microblogging server on ActivityPub with features such as emoji reactions, a customizable web UI, rich chatting, and much more!

</div>

<div style="clear: both;"></div>

# Links

### Want to get involved? Great!

- If you have the means to, [donations](https://opencollective.com/Firefish) are a great way to keep us going.
- If you know how to program in TypeScript, Vue, or Rust, read the [contributing](./CONTRIBUTING.md) document.
- If you know a non-English language, translating Firefish on [Weblate](https://hosted.weblate.org/engage/firefish/) help bring Firefish to more people. No technical experience needed!

### Links

- Donations:
  - OpenCollective: <https://opencollective.com/Firefish>
- Matrix space: <https://matrix.to/#/#firefish-community:nitro.chat>
- Official account: <a href="https://info.firefish.dev/@firefish" rel="me">`@firefish@info.firefish.dev`</a>
- Weblate: <https://hosted.weblate.org/engage/firefish/>

# Release cycle

We use a rolling release cycle now. Small updates will be frequently merged into the main branch, so please upgrade regularly at your convenience. We will use the commit date (e.g., 20240206) as the version number.

Any special operations required for an upgrade (e.g., installing new dependencies, editing docker-compose.yml) will be noted on [docs/notice-for-admins.md](docs/notice-for-admins.md) , so please check it before upgrading.

**As always, please take a backup first before starting update.** Whether it is a server snapshot backup or a complete database and file backup, it will be of great help to your recovery process.

# Getting started

This guide will work for both **starting from scratch** and **migrating from Misskey**.

<!-- ## Easy installers

If you have access to a server that supports one of the sources below, I recommend you use it! Note that these methods *won't* allow you to migrate from Misskey without manual intervention.

[![Install on Ubuntu](https://pool.jortage.com/voringme/misskey/3b62a443-1b44-45cf-8f9e-f1c588f803ed.png)](https://firefish.dev/firefish/ubuntu-bash-install)　　[![Install on the Arch User Repository](https://pool.jortage.com/voringme/misskey/ba2a5c07-f078-43f1-8483-2e01acca9c40.png)](https://aur.archlinux.org/packages/firefish)　　[![Install Firefish with YunoHost](https://install-app.yunohost.org/install-with-yunohost.svg)](https://install-app.yunohost.org/?app=firefish) -->

## Containerization

- [How to run Firefish with Docker](https://firefish.dev/firefish/firefish/-/blob/develop/docs/docker.md)
- [How to run Firefish with Kubernetes/Helm](https://firefish.dev/firefish/firefish/-/blob/develop/docs/kubernetes.md)

## Install Production Environment

For install production environment details, read [this document](https://firefish.dev/firefish/firefish/-/blob/develop/docs/install.md).

## Software Dependencies

- At least [NodeJS](https://nodejs.org/en/) v18.16.0 (v20/v21 recommended)
- At least [PostgreSQL](https://www.postgresql.org/) v12 (v16 recommended) with [PGroonga](https://pgroonga.github.io/) extension
- At least [Redis](https://redis.io/) v7
- Web Proxy (one of the following)
  - Caddy (recommended for new users)
  - Nginx (recommended)
  - Apache

### Optional dependencies

- [FFmpeg](https://ffmpeg.org/) for video transcoding
- Caching server (one of the following)
  - [DragonflyDB](https://www.dragonflydb.io/) (recommended)
  - [KeyDB](https://keydb.dev/)
  - Another [Redis](https://redis.io/) server

### Build dependencies

- At least [Rust](https://www.rust-lang.org/) v1.74
- C/C++ compiler & build tools
  - `build-essential` on Debian/Ubuntu Linux
  - `base-devel` on Arch Linux
- [Python 3](https://www.python.org/)

## Migrating from Misskey/FoundKey to Firefish

For migrating from Misskey v13, Misskey v12, and FoundKey, read [this document](https://firefish.dev/firefish/firefish/-/blob/develop/docs/migrate.md).

<p align="center"><img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" height="64" alt="API Server Template"></p>
<h3 align="center">API Server Template</h3>
<p align="center">Clean API Server template using Axum</p>
<p align="center">
    <a href="https://github.com/nkitan/api-server-template/blob/master/LICENSE.md"><img src="https://img.shields.io/badge/license-AGPL-blue.svg" alt="GNU-AGPL License"></a>
    <a href="https://github.com/nkitan/api-server-template/issues"><img src="https://img.shields.io/badge/contributions-welcome-ff69b4.svg" alt="Contributions Are Welcome"></a>
</p>

## Features

- High Performance REST API
- Shared Config State
- Environment Variable Support
- Automatically Generate and Serve OpenAPI JSON
- Uses Latest Version of Axum (0.8)

A list of upcoming / in-progress features can be found in the [TODO.md](TODO.md) file

## Requirements
1. PostgreSQL running in standalone / cluster mode with replication
2. Rust ~1.8
3. Keycloak ~23.0.6

## Usage

Setting up api-server-template is as easy as 
- configuring AXUM.env with details required for connecting to postgres and keycloak
- configuring .env with DATABASE URL
- running migrations
and finally running cargo run on the cloned directory

```sh
$ git clone https://github.com/nkitan/api-server-template
$ cd api-server-template
$ sqlx migrate run
$ cp AXUM.env.template AXUM.env
$ cargo run
```
The API can then be accessed at http://localhost:3030

## Maintainers

* [Ankit Das](https://github.com/nkitan)

## License

This project is licensed under the GNU AGPLv3 License - see the [LICENSE.md](LICENSE.md) file for details.
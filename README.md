<p align="center"><img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" height="64" alt="API Server Template"></p>
<h3 align="center">API Server Template</h3>
<p align="center">Clean API Server template using Axum</p>
<p align="center">
    <a href="https://github.com/nkitan/api-server-template/blob/master/LICENSE.md"><img src="https://img.shields.io/badge/license-AGPL-blue.svg" alt="GNU-AGPL License"></a>
    <a href="https://github.com/nkitan/api-server-template/issues"><img src="https://img.shields.io/badge/contributions-welcome-ff69b4.svg" alt="Contributions Are Welcome"></a>
</p>

## Features

- REST API
- Shared Config State
- Environment Variable Support
- Automatically Generate and Serve OpenAPI JSON
- Uses Latest Version of Axum (0.8)

## Usage

Setting up api-server-template is as easy as setting up AXUM.env and running cargo run on the cloned directory
```sh
$ git clone https://github.com/nkitan/api-server-template
$ cd api-server-template
$ cp AXUM.env.template AXUM.env
$ cargo run
```

## Maintainers

* [Ankit Das](https://github.com/nkitan)

## License

This project is licensed under the GNU AGPLv3 License - see the [LICENSE.md](LICENSE.md) file for details.

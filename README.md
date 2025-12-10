<h1>pheasant</h1>
pheasant is a http web server framework written in rust

[<img alt="crates.io" src="https://img.shields.io/crates/v/pheasant.svg?style=for-the-badge&color=E4004
6&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/pheasant)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-pheasant-495c9f?style=for-the-badge&logo=
docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/pheasant)
[<img alt="build:test" src="https://img.shields.io/github/actions/workflow/status/uraneko/pheasant/rust
-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/pheasant/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/pheasant?style=for-the-badge&lab
elColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/pheasant/blob/main/LICENSE)

## ToC
- [Design](#Design)
- [Features](#Features)
- [MSRV](#MSRV)
- [License](#License)

###
### Design
An HTTP server is too broad a program; anything the host system can do (calling another program, doing some calculations, db ops, fs ops, etc.,) the server should be able to do. 
That's why the main goal of this framework is to give the user smaller individual components that are kept as inter-indepent as possible and let the user write their own server implementations.
check out the main lib's public apis for a list of supported features.

### 
### Features

| Feature | PLANNED | TODO | ENHANCING | FIXING |
| :------------- | :--: | :--: | :--: | :--: |
| no_std compatibility         | x | | | |
| Origin server support        | | x | | |
| Http/1.1 support             | | x | | |
| Http/2 support               | x | | | |
| TLS/1.3 (Https) support      | x | | | |
| Services integration  | | x | | |
| Builtin services      | | x | | |
| Http/1.1 Pipelining   | x | | | | 
| websocket protocol    | x | | | |

###

* TODO: will start working on soon / mid-work on the basic features. There exists a proper statement of how and what would be implemented for this module to transition from TODO to ENHANCING 
* PLANNED: intend to work on at some time on the future. Only the statement of this intention exists.
* ENHANCING: basic features are done, at a stage of incorporating additional features when required 
* FIXING: basic features of the module are written, but there exist some bugs. The bugs severity depends on the module tags

###
### Usage

####
#### Install
```bash
cargo add pheasant_core
```

###
### MSRV 
1.88.0

###
### License 
<a href="LICENSE">MIT</a>

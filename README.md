<h1>pheasant</h1>
pheasant is a http web server framework written in rust.

An HTTP server is too broad a program; anything the host system can do (calling another program, doing some calculations, db ops, fs ops, etc.,) the server may need to do. 
That's why the main goal of this framework is to give the user smaller individual components that integrate well together (and preferably with external libs/crates) and let them write their own server implementations.


[<img alt="crates.io" src="https://img.shields.io/crates/v/pheasant.svg?style=for-the-badge&color=E4004
6&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/pheasant)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-pheasant-495c9f?style=for-the-badge&logo=
docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/pheasant)
[<img alt="build:test" src="https://img.shields.io/github/actions/workflow/status/uraneko/pheasant/rust
-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/pheasant/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/pheasant?style=for-the-badge&lab
elColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/pheasant/blob/main/LICENSE)

> [WARN] This readme is a work in progress.

## ToC
- [Goals](#Goals)
- [Crates](#Crates)
- [MSRV](#MSRV)
- [License](#License)

### 
### Goals

This framework aims to be:
* fully functional in a no_std state
* moderately low level a backend web framework

###
### Crates

| crate | features |
| :------------- | :--: |
| <a href="crates/pheasant_prologue">prologue</a> | http primitive types (Method, Protocol, Status),<br />http/1.1 client/server request/response parsing |
| <a href="crates/pheasant_uri">uri</a> | URL parser and URN parser |
| <a href="crates/pheasant_services">services</a> | Various middlewares, server/services pattern traits |
| <a href="crates/pheasant_socket">socket</a> | safe low level socket api bindings<br />only AF_UNIX and AF_INET addresses currently supported |
| <a href="crates/pheasant_sys">sys</a> | c-ffi calls, currently only has the socket module |

###
### usage

####
#### whole add

This includes all the framework crates.

```bash
cargo add pheasant
```

#### 
#### partial add
```bash
# replace * with the crate you need
cargo add pheasant_*
```

###
### msrv
pheasant aims to be compatible with at least this version of the rust compiler / cargo package manager: 1.88.0

###
### license 
<a href="LICENSE">MIT</a> only.

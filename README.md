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
- [Goals](#Goals)
- [Features](#Features)
- [MSRV](#MSRV)
- [License](#License)

###
### Design 

This table describes the implementation choices of this framework, it has to columns: 
- Framework: the logic this framework chooses to implement 
- Server: the logic this framework chooses to have the user implement 

| Logic | Framework | Server |
| :------------------------- | :-: | :-: |
| Data Types                 |  x  |     |
| Request Parsing            |  x  |     |
| Response Status Resolution |  x  |     |
| Response Building          |     |  x  |

###
### Goals

| Goal | \*Basic | Full | Extra | 
| :------------- | :--: | :--: | :--: | 
| Origin Server | x | | |
| Http/1.1 | x | | |
| Http/2 | | | |
| TLS/1.3 (Https) | | | |
| DataBases Integration | | | |
| Headers (Cors) | x | | |
| Headers (Cooks.) | x | | |
| Headers (Msg. Body Info.) | x | | |
| Headers (Date, Host) | x | | |

###


* `Basic` the feature is supported in a barebones manner, the api is still being worked out and may contain bugs.

* `Full` means the feature should be very close to feature completion and offers a robust/well tested api 

* `Extra` means that most bugs/issues have been ironed out and the current interest is user convenience and performance

### 
### Features
- http 1.1 request parsing + response auto generation
- http request redirection 
- http client/server error responses
- services as async functions 
- `get` attribute macro 

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

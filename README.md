<h1>pheasant</h1>
pheasant is a http web server framework written in rust

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
- [goals](#goals)
- [apis](#apis)
- [MSRV](#MSRV)
- [License](#License)

### 
### goals

This framework aims to be: 
* fully functional in a no_std state

### 
### apis

| api | crate | state |
| :------------- | :--: | :--: |
| <a href="crates/pheasant_uri">url scheme</a> | uri | working |
| <a href="crates/pheasant_prologue">http1.1 req/resp parsing</a> | prologue | working |
| <a href="crates/pheasant_prologue">http1.1 req/resp builders</a> | services
| <a href="crates/pheasant_socket">system socket api bindings (linux only)</a> | socket | clean-up |
| <a href="crates/pheasant_socket">no-std socket impl</a> | socket | clean-up |
| <a href="crates/pheasant_services">built-in middlwares</a> | services | extending |
| <a href="crates/pheasant_prologue">http primitive types</a> | prologue | working |
| <a href="crates/pheasant_services">server pattern logic</a> | services | working | 
| <a href="crates/pheasant_prologue">basic server socket</a> | prologue | working |

### 

clean-up: means the api needs a code refactor/clean-up, until then, the user may find it inconvenient or weirdly designed.

working: means the api is currently working and you can use it with no worries.

extending: means working + new features are being added.

###
### features
| feature | dependencies |
| :------ | :----------: |
| there are no crate features yet | - |

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

# Changelog

All notable changes to this project will be documented in this file. Commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/) specification, which drives the version bumps and the per-release sections below.



### [0.2.0](https://github.com/tirithen/clipper2c-sys/compare/v0.1.6...v0.2.0) (2026-05-05)

### BREAKING CHANGES

* bump rust edition to 2024 ([c9ba599](https://github.com/tirithen/clipper2c-sys/commit/c9ba599d97c3e918aefcb85661e40f0dfa0044ef))

### Bug Fixes

* handle wasm32-unknown-unknown in build.rs stdlib link step ([5b67a6d](https://github.com/tirithen/clipper2c-sys/commit/5b67a6d0b943eddbd4a6fa2a34b43a924d35fd29))
* adapt to Clipper2 2.0.1 api changes ([2ac5886](https://github.com/tirithen/clipper2c-sys/commit/2ac58863598137e8012dfcbc8fdccff7745d07a2))
* rendered URL and BREAKING CHANGES classification ([4dd3fdd](https://github.com/tirithen/clipper2c-sys/commit/4dd3fdd712003ff8c39f339c343f7e978c2c7c8a))

### Features

* update vendored Clipper2 to 2.0.1 ([e2f912d](https://github.com/tirithen/clipper2c-sys/commit/e2f912d805f1b52149568379efec4beecfbaa897))
* expose Minkowski sum/difference via FFI allowlist ([852b90b](https://github.com/tirithen/clipper2c-sys/commit/852b90b380d8c44d1d61e8f8649e1791d440b4d0))

### [0.1.6](https://github.com/tirithen/clipper2c-sys/compare/v0.1.5...v0.1.6) (2026-04-13)

This release contains performance improvements that reduces the memory use of the FFI code using references over copying data [6403217](https://github.com/tirithen/clipper2c-sys/commit/64032177d6b0afcd241b5b65f0c41b48a6d532db)

### Bug Fixes

* scale copied rect in rect scale functions ([912399c](https://github.com/tirithen/clipper2c-sys/commit/912399c3c2b7ac3012b3db7232051dc521062a78))

### [0.1.5](https://github.com/tirithen/clipper2c-sys/compare/v0.1.4...v0.1.5) (2025-06-17)


### Features

* use clipper c++ lib version 1.5.4 ([f888844](https://github.com/tirithen/clipper2c-sys/commit/f88884478d8e5b776dfd8623b4114d1151af6224))


### Bug Fixes

* use c++ memory allocator to match delete calls ([fb47275](https://github.com/tirithen/clipper2c-sys/commit/fb4727519179c0db2b08dc3a421b9c2b0447f811))

### [0.1.4](https://github.com/tirithen/clipper2c-sys/compare/v0.1.3...v0.1.4) (2024-07-30)


### Features

* update Clipper2 dep library to 1.4.0 + fixes ([87502f6](https://github.com/tirithen/clipper2c-sys/commit/87502f6628d37f1423bf8ae182bde5e74e8b27ed))

### [0.1.3](https://github.com/tirithen/clipper2c-sys/compare/v0.1.2...v0.1.3) (2024-07-01)


### Features

* add common derive macros to bindgen structs ([cee7912](https://github.com/tirithen/clipper2c-sys/commit/cee79122e6e15a8dfc0ddf0516e8724c97a168b9))

### [0.1.2](https://github.com/tirithen/clipper2c-sys/compare/v0.1.1...v0.1.2) (2024-06-01)


### Features

* **serde:** derive Serial./Deserial. on serde feat ([e15f63c](https://github.com/tirithen/clipper2c-sys/commit/e15f63ce48a6038a597cc7ef5a784c9d1968caf6))

### [0.1.1](https://github.com/tirithen/clipper2c-sys/compare/v0.1.0...v0.1.1) (2024-05-29)


### Features

* add area function bindings ([cb2cb8a](https://github.com/tirithen/clipper2c-sys/commit/cb2cb8a835b40a4fa44cb2eb6162f8a1cf57817c))
* add serde for ClipperPoint64 ([1e57950](https://github.com/tirithen/clipper2c-sys/commit/1e5795002ec17041a9f40cb22454f25f3567aae1))

## 0.1.0 (2024-04-28)


### Features

* add ffi bindings for clipper2c, and asic test ([12eed96](https://github.com/tirithen/clipper2c-sys/commit/12eed96d1e71089869ca72f67629ed3e67cc39f6))

# Rustup-distribution: the OffLine Rust toolchain packager
Implement a custom Rust offline installation package by adding a simple configuration file.

# Usage
Executing the build command will package the required toolchain into the Rust distribution package format, which is consistent with the format and usage of the official Rust offline distribution.
`/path/to/toolchain.yaml` The path to local configuration file.
`/path/to/package_dir` The path to store offline packages locally.
```shell
rustup-distribution package -s /path/to/toolchain.yaml -o /path/to/package_dir
```

# Example
Use the following format to configure the offline package toolchain
```yaml
RUSTUP_DIST_SERVER: http://example.com
RUSTUP_UPDATE_ROOT: http://example.com
TARGETS:
  - target: x86_64-pc-windows-msvc
    channel: nightly
    date: 2023-06-15
    profile: default
EXTEND_TOOLS:
  - name: grcov
    version: 0.8.18
  - name: rust-code-analysis-cli
    version: 0.0.24
```

# Compiling from Source
Requirements
Cargo requires the following tools and packages to build:

cargo and rustc
A C compiler for your platform
git (to clone this repository)
Other requirements:

The following are optional based on your platform and needs.

pkg-config — This is used to help locate system packages, such as libssl headers/libraries. This may not be required in all cases, such as using vendored OpenSSL, or on Windows.

OpenSSL — Only needed on Unix-like systems and only if the vendored-openssl Cargo feature is not used.

This requires the development headers, which can be obtained from the libssl-dev package on Ubuntu or openssl-devel with apk or yum or the openssl package from Homebrew on macOS.

If using the vendored-openssl Cargo feature, then a static copy of OpenSSL will be built from source instead of using the system OpenSSL. This may require additional tools such as perl and make.

On macOS, common installation directories from Homebrew, MacPorts, or pkgsrc will be checked. Otherwise it will fall back to pkg-config.

On Windows, the system-provided Schannel will be used instead.

LibreSSL is also supported.

Optional system libraries:

The build will automatically use vendored versions of the following libraries. However, if they are provided by the system and can be found with pkg-config, then the system libraries will be used instead:

libcurl — Used for network transfers.
libgit2 — Used for fetching git dependencies.
libssh2 — Used for SSH access to git repositories.
libz (aka zlib) — Used for data compression.
It is recommended to use the vendored versions as they are the versions that are tested to work with Cargo.
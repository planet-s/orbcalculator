# OrbCalculator

[![Build status](https://gitlab.redox-os.org/redox-os/orbcalculator/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbcalculator/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Calculator written in rust, developed with OrbTk https://gitlab.redox-os.org/redox-os/orbtk".

<img alt="Redox" height="300" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Platforms

* Redox OS (native | cargo-node)
* Linux (native | cargo-node)
* macOS (native | cargo-node)
* Windows (native | cargo-node)
* openBSD (not tested, but should work)
* Web (cargo-node)
* Android (native planned after 0.3 | cargo-node)
* iOS (native planned after 0.3 | cargo-node planned after 0.3)
* Ubuntu Touch (native planned  after 0.3 | cargo-node planned for 0.3)

## Run

You can start the calculator by executing the following command:

```text
cargo run --release
```

## Run with cargo-node

To run the calculator on as browser, electron or cordova app you have to install

```text
cargo install -f cargo-node
```

Before you could use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You could download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatic.

### Start

You can start the widgets example by executing the following command:

* Run as browser app:

```text
cargo node run --browser
```

* Run as electron app:

```text
cargo node run --electron
```

* Run as cordova app on android:

```text
cargo node run --android
```

## License

Licensed under MIT license ([LICENSE](LICENSE)).
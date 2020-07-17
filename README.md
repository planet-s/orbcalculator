# OrbCalculator

[![Build status](https://gitlab.redox-os.org/redox-os/orbcalculator/badges/master/pipeline.svg)](https://gitlab.redox-os.org/redox-os/orbcalculator/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Calculator written in rust, developed with OrbTk https://github.com/redox-os/orbtk.git.

<img alt="Redox" height="300" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Platforms

* Redox OS
* Linux
* macOS
* Windows (wip)
* openBSD (not tested, but should work)
* Android (planned)
* iOS (planned)
* Ubuntu Touch (planned)

## Run

You can start the calculator by executing the following command:

```shell
cargo run --release
```

To start the calculator with the light theme execute following command:

```shell
cargo run --release --features light
```

## License

Licensed under MIT license ([LICENSE](LICENSE)).
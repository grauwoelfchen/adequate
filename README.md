# Adequate

[![pipeline](
https://gitlab.com/grauwoelfchen/adequate/badges/master/pipeline.svg)](
https://gitlab.com/grauwoelfchen/adequate/commits/master) [![coverage](
https://gitlab.com/grauwoelfchen/adequate/badges/master/coverage.svg)](
https://gitlab.com/grauwoelfchen/adequate/commits/master)

A yet another validation library provides a macro inspired by [Accord](
https://github.com/ChrisBuchholz/accord).


## Repos

The source code is hosted in several repos, but developed mainly on [
GitLab.com](https://gitlab.com/grauwoelfchen/adequate).

The merge/pull requests or issues on any repository are welcomed.

* https://gitlab.com/grauwoelfchen/adequate
* https://github.com/grauwoelfchen/adequate
* https://git.sr.ht/~grauwoelfchen/adequate


## Installation

```zsh
% cargo install adequate
```

## Usage

See `src/validation` directory for validators.

```rust
use adequate::validation::length;

// inputs
let fullname = "Albrecht Dürer".to_string();
let username = "albrecht".to_string();

let result = validate! {
    "fullname" => fullname => [length::max(3)],
    "username" => username => [length::within(3..9)]
};
assert!(result.is_err());
```

### Validations

###### Length

* max
* max_if_present
* min
* min_if_present
* within


## Build

Check `make help`

```zsh
# debug build
% make build:debug
```

## Development

### Verify

```zsh
# check code using all verify:xxx targets
% make verify:all
```

### Test

```zsh
% make test
```

### CI

Run CI jobs on local docker conatiner (Gentoo Linux) using gitlab-runner.  
See `.gitlab-ci.yml`.


```zsh
# install gitlab-runner into .tools
% .tool/setup-gitlab-runner

# prepare environment variables for CI via .env.ci
% cp .env.ci.sample .env

# e.g. test (see .gitlab-ci.yml)
% .tool/ci-runner test
```


## Release

All notable released changes of this package will be documented in CHANGELOG
file.

### Unreleased commits

[v0.1.0...master](
https://gitlab.com/grauwoelfchen/adequate/compare/v0.1.0...master)


## License

```text
Adequate
Copyright 2020 Yasuhiro Яша Asaka

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

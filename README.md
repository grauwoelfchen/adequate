# Adequate

[![pipeline](
https://gitlab.com/grauwoelfchen/adequate/badges/master/pipeline.svg)](
https://gitlab.com/grauwoelfchen/adequate/commits/master) [![coverage](
https://gitlab.com/grauwoelfchen/adequate/badges/master/coverage.svg)](
https://gitlab.com/grauwoelfchen/adequate/commits/master)

Yet another validation library which is inspired by [Accord](
https://github.com/ChrisBuchholz/accord)


## Repository

https://gitlab.com/grauwoelfchen/adequate


## Installation

```zsh
% cargo install adequate
```

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

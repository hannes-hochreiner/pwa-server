# PWA-Server

A server for progressive web apps.

## Configuration

PWA-Server expects the following environment variables:

|Name | Description|
|----|----|
|ROOT_DIR| location of the directory containing the static content served at "/"|
|CONFIG_DIR| location of the directory containing the static content served at "/config"|

If the root directory contains paths that coincide with the config directory, the files in the config directory will be served.

## License

This work is licensed under the MIT license.

`SPDX-License-Identifier: MIT`

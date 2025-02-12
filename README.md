# Finance Data Harvester

[![License](https://img.shields.io/github/license/felipet/lacoctelera_backend?style=flat-square)](https://github.com/felipet/finance_data_harvester/blob/main/LICENSE)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/felipet/finance_data_harvester/rust.yml?style=flat-square&label=CI%20status)


A command line tool to manage the Finance Data Harvest Library.

## Introduction

This binary crate implements a simple command line tool that works as frontend of the [Finance Data Harvest Library](https://github.com/felipet/data_harvest). Along the tool, a **systemd** service and timer are included to automate the collection of data periodically.

## Usage

The tool expects to connect to a database server in which the new harvested data will be pushed. The connection string is read from an environment variable named `DATABASE_URL`. Check the [`sqlx`](https://crates.io/crates/sqlx) crate for more information about how to define a connection string.

```
Usage: finance_harvester [OPTIONS]

Options:
  -d, --debug...                           Turn debugging information on
      --refresh-short <ticker list | all>  Refresh short positions.
      --service                            Run as a systemd service
  -h, --help                               Print help
  -V, --version                            Print version
```

### Logs

The tool includes [`tracing`](https://crates.io/crates/tracing) support. By default, only the error messages are logged. To include more information call the tool using `-d` for `INFO` or add more `d`s  to increase the verbosity. Have a look to `telemetry.rs` for a full list of the supported verbosity levels.

When the tool is ran as a systemd service, log messages are redirected to `journald`. The verbosity level applies the same. To check the logs, use:

```
journalctl --user -xeu finance-harvester.service
```

*Note:* remove `--user` if you plan to run the service as a system service.

## Systemd Installation

The tool performs a one time collection of the specified data. If you plan to collect data periodically, a **systemd timer** is the way to go. Under the *systemd* folder, you'll find two files:

- ***finance_harvester.service***: The main service descriptor.
- ***finance_harvester.timer***: The timer descriptor.

I suggest installing these as a regular user. To do so, you can copy both files to `~/.config/systemd/user` and then enable the timer:


```bash
$ systemctl --user daemon-reload
$ systemctl --user enable --now finance_harvester.timer
```

Finally, you can test the service manually using:

```bash
$ systemctl --user start finance_harvester.service
```

Or check that the timer is properly setup and wait for it:

```bash
$ systemctl --user list-timers
```

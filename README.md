<div align="center">
  <h1><code>tdb</code></h1>
  <p><strong>Simple Talis MSSQL CLI.</strong></p>
</div>

## About

1. **All traffic to and from servers is encrypted** using the native Windows TLS implementation (Schannel).

2. Database authentication is done using integrated authentication (Active Directory).

3. Queries are only checked by the database.

4. No database connections are pooled because the lifetime of this application begins and ends on the command line.

## Help

```sh
tdb help
tdb help <SERVER>
```

## Usage

```sh
tdb <SERVER> <DATABASE> <OP> <TABLE> [args]
```

For example:

```sh
# quotes are required, or the command line will split your arguments
tdb dev-east bid01 select staff --where "loginuserid = 'tbyron'"
tdb dev-east bid01 s staff -w "loginuserid = 'tbyron'"
```

```sh
tdb dev-east bid01 update staff --where "loginuserid='tbyron'" --set "pin='1212'"
tdb dev-east bid01 u staff -w "loginuserid='tbyron'" -s "pin='1212'"
```

## Config

The main config `tdb.toml` should be located in the same directory as the executable. All config files use the [TOML](https://github.com/toml-lang/toml) file format.

### Custom config file

You may create custom config files in any directory. This can be useful for creating identical test accounts across multiple databases. The command line argument `-c` or `--config` must be passed to the CLI to load a custom config file.

> **Note**: `Server` connection URLs will only be read from the main config file `tdb.toml`.

```toml
# File name: my-staff.toml

[Staff]
LoginUserId = 'jdoe'
PIN = '1111'
FirstName = 'John'
LastName = 'Doe'
NTUserName = 'john.doe'
EmailAddress = 'jdoe@talisclinical.com'
SSOUserId = 'aaaaaaaa-1111-bbbb-2222-cccccccccccc'

# …
```

The custom config file may then be passed to the CLI like so:

```sh
tdb --config '.\my-staff.toml' dev-east bid01 insert staff
# or
tdb -c '.\my-staff.toml' dev-east bid01 insert staff
```

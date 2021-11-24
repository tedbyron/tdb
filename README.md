<div align="center">
  <h1><code>tdb</code></h1>
  <p><strong>Simple Talis database commands.</strong></p>
</div>

## About

1. **All traffic to and from servers is encrypted** with native TLS implemented by:

   - Schannel on Windows
   - Secure Transport on macOS
   - OpenSSL on other platforms

2. No network connections are made until after queries are checked at runtime.

3. No database connections are pooled because the lifetime of this application begins and ends on the command line.

## Help

```sh
tdb help
tdb help <SERVER>
```

## Usage

```sh
tdb <SERVER> <DATABASE> <OP> <TABLE> [args]
# e.g.
tdb dev-east bid01 select staff --where "loginuserid = 'tbyron'"
tdb dev-east bid01 u staff -w "loginuserid='tbyron'" -s "pin='1212'"
```

## Config

The main config `tdb.toml` should be located in the same directory as the executable. All config files use the [TOML](https://github.com/toml-lang/toml) file format.

### Custom config file

You may create custom config files in any directory. The command line argument `-c` or `--config` must be passed to the CLI to load a custom config file.

> **Note**: `Server` connection URLs will only be read from the main config file `tdb.toml`

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
tdb --config 'my-staff.toml' dev-east bid01 insert staff
# equivalent to
tdb -c 'my-staff.toml' dev-east bid01 insert staff
```

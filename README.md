<div align="center">
  <h1><code>tdb</code></h1>
  <p><strong>Simple Talis MSSQL CLI.</strong></p>
</div>

## About

1. **All network traffic is encrypted** using the native Windows TLS implementation (Schannel).

2. **Queries are only checked by the database**.

3. **No network connections are pooled** because the lifetime of this application begins and ends on the command prompt.

4. Authentication is performed using Windows authentication (SSPI).

5. All network connections currently use TCP/IP. Support for named pipes instead of TCP/IP may be implemented in the future.

6. Exact ports must be specified or else the program will attempt to use a default port of `1433`.

## Help

```sh
tdb help
tdb help <SERVER>
```

## Usage

Database servers used in commands are defined in the `tdb.toml` [config](#config) file.

```sh
tdb <SERVER> <DATABASE> <OPERATION> <TABLE> [OPTIONS]
```

For example:

```sh
# quotes are required, or the command prompt will split your arguments
tdb dev-east bid01 select staff --where "loginuserid = 'example'"
```

There are short versions of many arguments:

```sh
tdb dev-east bid01 s staff -w "loginuserid='example'"
```

## Config

The main config `tdb.toml` should be located in the same directory as the executable. All config files use the [TOML](https://github.com/toml-lang/toml) file format.

### Server name aliases

The names of items in the `tdb.toml` `Servers` table can be changed. For example, the below:

```toml
[Servers]
e = "example.com"
# …
```

Would be a little bit faster to type out in a full command:

```sh
tdb e bid01 s staff -w "loginuserid='example'"
#   ^
```

### Custom config file

You may create custom config files in any directory. This can be useful for inserting identical test accounts across multiple databases.

> **Note**: `Server` connection URLs will only be read from the main config file `tdb.toml`.

```toml
# File path: ./my-staff.toml
[Staff]
LoginUserId = 'jdoe'
PIN = '1111'
FirstName = 'John'
LastName = 'Doe'
NTUserName = 'john.doe'
EmailAddress = 'jdoe@example.com'
SSOUserId = 'aaaaaaaa-1111-bbbb-2222-cccccccccccc'
# …
```

The custom config file may then be passed to the CLI using the `-c, --config` argument:

```sh
tdb --config './my-staff.toml' dev-east bid01 insert staff
```

# Basque - execute shell commands from SQLite

**This is a work in progress and doesn't work yet!**

## Remaining work

- [x] Prototype to confirm `no_mangle` will work and SQLite can load the Rust-built module
- [ ] Generate `sqlite3ext.h` API struct with rust-bindgen
- [x] Switch to Cargo for building
- [ ] Fix all the places I'm cheating and using `u64` instead of pointer types
- [ ] Actually learn Rust

[![CircleCI](https://circleci.com/gh/pnc/basque.svg?style=svg)](https://circleci.com/gh/pnc/basque)

## Theoretical example usage

```sql
sqlite> select count(*) - 1 from (select basque_cmd("ps -o command"));
14
```

## Development

You will need a copy of SQLite with module loading enabled. The default (`/usr/bin`) installation on macOS does not, but the Homebrew one does. To use the Homebrew one:

```
export PATH=/usr/local/opt/sqlite3/bin:$PATH
```

If module loading is disabled, you'll get this error when you try to run `.load ./libbasque`:

```
Error: unknown command or invalid arguments:  "load". Enter ".help" for help
```

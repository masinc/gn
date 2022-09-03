# gn

A gron-like tool made in Rust. \
Support format for JSON, JSON5, YAML and TOML.

Inspired by [gron](https://github.com/TomNomNom/gron)

# Usage

This reads FILEPATH and outputs it in gron format.

```
gn <FILEPATH>
```

If there is no argument or "-", it reads from stdin.

```
cat Cargo.toml | gn
```

To determine the format, first try to determine by extension. Next, if it cannot be determined by extension, it attempts to deserialize to each format.


# License

MIT or Apache-2.0

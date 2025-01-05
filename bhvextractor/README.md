# bhvextractor
simple tool written in rust to determine every used behavior in a folder of lvl64 files, optionally with bhv names if you input a config file. (config file trol unimpl)

**in its current state, this just prints out every bhv, i reccommend outputting it to a file.**

## use
memdump should be a .bin dump of the memory from project64

leveldir should be the directory where your lvl64s are.

if seg is "true", then it will output segmented addresses, if it's anything else, it'll output the virtual ram addresses.

you can get these files using sm64 rom manager.

```
cargo run -- <memdump> <leveldir> <seg> # for testing
./bhvextractor <memdump> <leveldir> <seg> # if you have a binary
```

**planned:**

you can specify another argument for the config file. (**not yet**)
if there isn't a config file specified, it'll only tell you the used bhvs.
```
./bhvextractor <leveldir> <config>
```


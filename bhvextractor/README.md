# bhvextractor
simple tool written in rust to determine every used behavior in a folder of lvl64 files, optionally with bhv names if you input a config file.

**in its current state, this just prints out every bhv, i reccommend outputting it to a file.**

also, it doesn't work. it says b3313 has ~8,000 bhvs :troll:

## use
leveldir should be the directory where your lvl64s are.

you can get these files using sm64 rom manager.

```
cargo run -- <leveldir> # for testing
./bhvextractor <leveldir> # if you have a binary
```

you can specify another argument for the config file.
if there isn't a config file specified, it'll only tell you the used bhvs.
```
./bhvextractor <leveldir> <config>
```


# cdialog
simple tool that allows exporting dialog from sm64 roms as C code for decomp.

does not require rm!

special thanks to the local doe, this is based on BIFE!

## usage
```
cargo run -- <rompath> # for testing
./cdialog <rompath> # if you have a binary
```

you can specify another argument for the output.
if there isn't an output name specified, it outputs as `dialog.c`
```
./cdialog <rompath> <outpath>
```

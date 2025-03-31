# lvl64exporter

the time has finally come where i have to write C# code.

thankfully, it isn't too hard!
this simple tool exports all levels (in lvl64 format) from the input rom into the specified directory.

## usage
make sure the `leveltable.json` file is in the current working directory.

if outdir already exists, the program exits.
it will not overwrite any existing folders or files.

template:

```
./lvl64exporter <rompath> <outdir>
```

example:

```
./lvl64exporter ./b3313\ copped.z64 ./beeeeeeeeeeee/
```

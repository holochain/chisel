# chisel

Swiss-army knife CLI tool for making more sense of Holochain logs of all kinds. Carve away at the Uncarved Block (of raw log text).

## Usage

chisel reads logs from stdin and writes the transformed output to stdout, when possible. It is intended to be used with command-line pipes.

### try-o-rama demuxer

The only command available currently is `chisel tryorama demux`. This takes a log file from a [try-o-rama](https://github.com/holochain/try-o-rama) test run and splits (demultiplexes) it into one file per conductor.

For example, if `npm test` runs your try-o-rama test suite, you might prefer to run it such that prints the output to a file as well as stdout:

```
npm test |& tee out.txt
```

With the log file in hand, you can pipe it through chisel to split it into several files. This output goes directly to a handful of files at a directory you specify with `-d`:

```
cat out.txt | chisel tryorama demux -v -d some/dir/that/exists
```

If you don't care to keep your raw logs around, you can also pipe the log output directly into chisel for a more streamlined workflow:

```
npm test |& chisel tryorama demux -v -d some/dir/that/exists
```

If you still want to see the raw logs in your terminal, you can split the stream with tee like so:

```
npm test |& tee >(chisel tryorama demux -v -d some/dir/that/exists)
```

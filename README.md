# xkcd-ascii
don't mind the code, it's _completly rusty_
```
Converts xkcd comics to ascii

USAGE:
    xkcd-ascii [id]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <id>    Id of xkcd comic. If not provided then a random comic is fetched.

```

## how to build

```bash
git clone https://github.com/HKGx/xkcd-ascii.git
cd xkcd-ascii
cargo build
```

the files should be somewhere in the target directory

## how to use

simply invoke

```bash
./xkcd-ascii 
```


you can also specify which comic do you want to fetch by doing e.g.

```bash
./xkcd-ascii 5
```

## proof of build

[![asciicast](https://asciinema.org/a/2GPif9bylgYVWHoRG1hOV1lhR.svg)](https://asciinema.org/a/2GPif9bylgYVWHoRG1hOV1lhR)

## proof of use

[![asciicast](https://asciinema.org/a/CqSZH6MTTnNKTjhU101C0MlFC.svg)](https://asciinema.org/a/CqSZH6MTTnNKTjhU101C0MlFC)
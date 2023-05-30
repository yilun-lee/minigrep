# minigrep 

This is my minigrep tool made only for learning rust. It is aimed to provided similar functionality with [ripgrep](). But for now it is still work in progress. Run slow, buggy and refactor-required. There are currently two crate here. First is argtool, for parse sys arg, used by minigrep. The other is grep, the main program for minigrep.


## grep

### Installation

```shell
# cd to this repo
cargo build
# This is binary location
./target/release/minigrep
```

### Current usage

#### Simple usage:

Basic pattern

```shell
./minigrep "YourPattern" "YourDir/*"
```

Print all fn in codes. 
 
```shell
./target/release/minigrep "fn" "minigrep/*" 
```

Print help

```shell
./target/release/minigrep -h
./target/release/minigrep --help
```

#### More usage:

<style>
    r {color: Red}
</style>

Print with line number and the following two line. 

```shell
./target/release/minigrep "fn" "minigrep/*" -n -B 2
```

Multiple match: struct, trait, and enum

```shell
./target/release/minigrep "trait" "minigrep/*" -n  -M "struct" -M "enum" -B 3 -A 1
```

Extract match pattern, here the pattern is function name
🪲: There are some wrong behavior in extract, need capture group. If parenthesis is added, that is ```"fn [A-Za-z0-9_]+\("``` Some of matched line will be empty.

```shell
./target/release/minigrep "fn" "minigrep/*" -n -E "fn [A-Za-z0-9_]+"
```

Replace ```->``` with ```==>```
🪲: if replacer is ```-->``` there will be error. `-` need to be escaped. We need ```"\->"``` for ```-R``` only, there is no need to specify it twice.

```shell
./target/release/minigrep "\->" "minigrep/*" -n -R "\->" -r "==>"
```


## argtool
go to [github page](https://yilun-lee.github.io/minigrep/argtool/index.html) generated by rust docs



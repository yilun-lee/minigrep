
BASEDIR="$(git rev-parse --show-toplevel)"
cd "$BASEDIR"

cargo build -p grep --release

#  print help 
cargo run -p grep -- --help

#  test match 
cargo run -p grep -- \
    "println" /Users/sox/CODE/minigrep/grep/src/main.rs \
    -n -M "\{" -M "\}"

#  test extract 
cargo run -p minigrep -- \
    "println" /Users/sox/CODE/minigrep/minigrep/src/main.rs \
    -n -E "\{" -E "\}"

#  test replace 
cargo run -p minigrep -- \
    "fn" "/Users/sox/CODE/*" \
    -n -R "\{" -r "<"  -R "\}" -r ">" -T 1

#  test no file error 
cargo run -p grep -- \
    "println" /Users/sox/CODE/minigrep/grdsafiosajfioep/src/main.rs 




# pprof
cargo run -p grepprof -- \
    "fn" "/Users/sox/CODE/minigrep/*" \
    -n -R "\{" -r "<"  -R "\}" -r ">" -R "\->" -r "==>" 





$BASEDIR/target/release/grep \
    "fn" "/Users/sox/CODE/minigrep/grep/*" \
    -n -B 4

$BASEDIR/target/release/grep \
    "fn" "/Users/sox/CODE/minigrep/grep/*" \
    -n -R "\->" -r "==>" -B 4




hyperfine --runs 10 --warmup 5 --ignore-failure \
    'bash scripts/minigrep.sh' 'bash scripts/ripgrep.sh' 'bash scripts/grep.sh'



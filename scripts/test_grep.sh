
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
cargo run -p grep -- \
    "println" /Users/sox/CODE/minigrep/grep/src/main.rs \
    -n -E "\{" -E "\}"

#  test replace 
cargo run -p grep -- \
    "println" /Users/sox/CODE/minigrep/grep/src/main.rs \
    -n -R "\{" -r "<"  -R "\}" -r ">" -t 1

#  test no file error 
cargo run -p grep -- \
    "println" /Users/sox/CODE/minigrep/grdsafiosajfioep/src/main.rs 



$BASEDIR/target/release/grep \
    "fn" "/Users/sox/CODE/minigrep/grep/*" \
    -n -B 4

$BASEDIR/target/release/grep \
    "fn" "/Users/sox/CODE/minigrep/grep/*" \
    -n -R "\->" -r "==>" -B 4

$BASEDIR/target/release/grep \
    "fn" "/Users/sox/CODE/minigrep/grep/*" \
    -n -E "fn\s+([A-Za-z0-9_]+)"


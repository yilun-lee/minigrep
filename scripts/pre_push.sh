
# ADD these to pre-push.sample
# get base
BASEDIR="$(git rev-parse --show-toplevel)"
cd "$BASEDIR"

# add doc
cargo doc --no-deps --workspace  
rm -rf docs || true
cp -r target/docs .
git add docs 
git commit -m "add docs" 



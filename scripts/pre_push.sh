
# ADD these to pre-push.sample
# get base
# BASEDIR="$(git rev-parse --show-toplevel)"
# cd "$BASEDIR"

# add doc
rm -rf target/doc || true
cargo doc --no-deps --workspace  
cp -r target/doc docs
git add docs 
git add -u
git commit -m "add docs" 



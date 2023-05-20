
# https://blog.devgenius.io/automate-unit-tests-before-each-commit-by-git-hook-f331f0499786

##### ADD these to .git/hooks/pre-push
# BASEDIR="$(git rev-parse --show-toplevel)"
# cd "$BASEDIR"
# bash scripts/pre_push.sh

# add doc
rm -rf target/doc || true
cargo doc --no-deps --workspace  

rm -rf docs || true
cp -r target/doc docs

git add docs 
git add -u
git commit -m "add docs" 



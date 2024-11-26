# Remove redundant CI workflow files

git rm .github/workflows/rust.yml
git rm .github/workflows/ci.yml
git rm anya-enterprise/.github/workflows/rust.yml
git rm anya-enterprise/.github/workflows/ci.yml

# Commit the changes
git commit -m "Remove redundant rust.yml and ci.yml workflows after consolidation"

# Push the changes to the main branch
git push origin main 
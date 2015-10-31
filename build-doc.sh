if [ "$TRAVIS_BRANCH" = 'master' ] && [ "$TRAVIS_PULL_REQUEST" = 'false' ] && [ "$TRAVIS_RUST_VERSION" = 'rustc 1.4.0 (8ab8581f6 2015-10-27)' ]; then
  # Fetch the docs
  git clone https://github.com/ryantaylor/vault.git docs --branch gh-pages
  
  # TODO: Custom stylings

  # Doc the crate and its dependencies
  cargo doc --features=ffi
  # Update the docs
  cp -Rf target/doc/* docs

  cd docs && git add --all
  git config user.name "travis"
  git config user.email "ryan@ryantaylordev.ca"
  git commit -m "(docs-autogen) ${TRAVIS_REPO_SLUG}."
  git push -q "https://${TOKEN}:x-oauth-basic@github.com/ryantaylor/vault.git" gh-pages
fi

echo 'doc upload completed'
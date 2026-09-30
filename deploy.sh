#!/bin/bash

if [ -z "$1" ] ; then
  echo "No argument supplied. Allowed: local, dev, prod"
  exit 1
fi

echo Deleting build/...
rm -rf build

echo Building...

# Used to use --release here, but now debug builds are fast and there's not much point.
cargo run -- --skip-serve --$1

echo deploying to www@main:/srv/www/ppsspp.org/$1

rsync -avh build www@main:/srv/www/ppsspp.org/$1 --delete-after

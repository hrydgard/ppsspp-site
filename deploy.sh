#!/bin/bash

if [ -z "$1" ] ; then
  echo "No argument supplied. Allowed: local, dev, prod"
  exit 1
fi

echo Deleting build/...
rm -rf build

echo Building...

cargo run -- --skip-serve

echo deploying to www@main:/srv/www/ppsspp.org/$1

rsync -av build www@main:/srv/www/ppsspp.org/$1

# TODO: Switch back to local automatically, instead of using switch.sh?


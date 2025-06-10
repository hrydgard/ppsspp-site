#!/bin/bash

if [ -z "$1" ] ; then
  echo "No argument supplied. Allowed: local, dev, prod"
  exit 1
fi

echo Deleting build/...
rm -rf build

echo Building...

cargo run --release -- --skip-serve --$1

echo deploying to www@main:/srv/www/ppsspp.org/$1

rsync -avh build www@main:/srv/www/ppsspp.org/$1 --delete-after

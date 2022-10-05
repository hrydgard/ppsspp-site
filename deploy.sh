if [ -z "$1" ] ; then
  echo "No argument supplied. Allowed: local, dev, prod"
  exit 1
fi

echo deploying to www@main:/srv/www/ppsspp.org/$1

cp env.$1.js env.js && npm run build && rsync -av build www@main:/srv/www/ppsspp.org/$1

# TODO: Switch back to local automatically, instead of using switch.sh?
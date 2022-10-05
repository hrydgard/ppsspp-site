# Use deploy.js to actually deploy. This is for convenience after deploy

if [ -z "$1" ] ; then
  echo "No argument supplied. Allowed: local, dev, prod"
  exit 1
fi

cp env.$1.js env.js && echo Switched to environment $1

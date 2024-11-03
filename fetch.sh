scp util/dirtree-json www@main:/tmp
scp util/dirtree-json www@biz:/tmp
ssh www@main 'chmod u+x /tmp/dirtree-json; /tmp/dirtree-json --root /srv/www/ppsspp.org/pub/files' > data/downloads.json
ssh www@biz 'chmod u+x /tmp/dirtree-json; /tmp/dirtree-json --root /srv/www/central.ppsspp.org/goldfiles' > data/downloads_gold.json

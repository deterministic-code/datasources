set -e
mysql --skip-ssl -h mysql -uroot -proot -e "DROP DATABASE IF EXISTS datasources; CREATE DATABASE datasources; GRANT ALL PRIVILEGES ON datasources.* TO 'ds'@'%';"
PGPASSWORD=ds dropdb -h postgres -U ds --if-exists --force datasources
PGPASSWORD=ds createdb -h postgres -U ds datasources

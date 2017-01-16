#!/bin/bash
if [ -e "keys" ]
then
  echo "keys dir already exists, please delete before re-runninig this script"
  exit -1
fi
mkdir keys
openssl genrsa -des3 -passout pass:x -out keys/server.pass.key 2048
openssl rsa -passin pass:x -in keys/server.pass.key -out keys/server.key
rm keys/server.pass.key
openssl req -new -key keys/server.key -out keys/server.csr
openssl x509 -req -days 365 -in keys/server.csr -signkey keys/server.key -out keys/server.crt
ln -s "$(pwd)/keys" "$(pwd)/target/release/"
ln -s "$(pwd)/keys" "$(pwd)/target/debug/"
echo "\n\n\n"
echo "On linux, to support self signed cert in chrome and postman, you must import the cert to chrome"
echo "in chrome, go to \"chrome://settings/certificates \""
echo "Click Manage Certificates, on the authorities tab, check for an existing folder/cert for"
echo "localhost delete if it already exists, then click import, navigate to server.crt, and import"
echo "close every single chrome process and children like hangouts, signal, then restart"
echo "navigate to https site using localhost if that is what you used for CN=, be sure postman uses"
echo "localhost and not 127.0.0.1"
echo "that is all! :)"

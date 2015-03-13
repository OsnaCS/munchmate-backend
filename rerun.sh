#!/bin/sh

cargo build
if [ $? = 0 ]
then
    echo "--------- Restarting Webserver ---------"
    killall -q munchmate-backend;
    . ./set_env.sh
    PORT=1338 ./target/debug/munchmate-backend &
fi
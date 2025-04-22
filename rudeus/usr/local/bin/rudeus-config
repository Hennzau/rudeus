#!/bin/sh

if [ -d "/etc/rudeus/default/" ]; then
    cp -r /etc/rudeus/default/* .
    echo "Content copied from /etc/rudeus/default/ to the current directory."
else
    echo "The directory /etc/rudeus/default/ does not exist."
    exit 1
fi

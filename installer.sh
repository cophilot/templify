#!/bin/bash

url=""

# save the file from the url and safe it as an executable for the user so that it can be run
wget $url -O /tmp/installer.sh
chmod +x /tmp/installer.sh

# run the installer


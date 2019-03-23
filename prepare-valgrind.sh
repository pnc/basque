#!/bin/bash
set -ex

echo "deb http://deb.debian.org/debian-debug/ unstable-debug main" > /etc/apt/sources.list.d/source.list
apt-get update
apt-get install -y clang curl sqlite3-dbgsym sqlite3 libsqlite3-dev valgrind || echo "Ignoring installation problems due to https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=923942"
curl https://sh.rustup.rs -sSf | bash -s -- -y

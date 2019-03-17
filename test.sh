#!/bin/bash

set -ex

echo '.load ./target/debug/libbasque' | sqlite3

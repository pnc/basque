#!/bin/bash

set -ex

valgrind --leak-check=full --keep-debuginfo=yes --trace-children=yes ./test.sh

#!/bin/bash

set -ex

rustc --crate-type=dylib basque.rs

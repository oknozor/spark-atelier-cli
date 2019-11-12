#!/bin/bash

cargo build
mv target/debug/next test/example
cd test/example
./next
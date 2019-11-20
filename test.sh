#!/bin/bash

cargo build
mv target/debug/next spark-atelier-cli-test/
cd spark-atelier-cli-test
touch pouet
./next
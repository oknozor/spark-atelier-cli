#!/bin/bash

cargo build
mv target/debug/foreman spark-atelier-cli-test/
cd spark-atelier-cli-test
touch pouet
echo "Submitting test with the following state : "
./foreman info
./foreman next
echo "New state : "
./foreman info
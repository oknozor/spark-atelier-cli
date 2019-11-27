#!/bin/bash

cargo build
mv target/debug/foreman spark-atelier-cli-test/
cd spark-atelier-cli-test
./foreman init --hard "Team SparkAttack"
./foreman info
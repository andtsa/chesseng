#!/bin/sh

path="$(date)"
cp -r ./target/criterion/ "./research/reports/$path/"
cp -r ./target/iai/ "./research/reports/$path/iai/"


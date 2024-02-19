#!/bin/sh

set -e
sed "/^\/\/!/d" -i src/lib.rs
mv src/lib.rs src/lib_new.rs

IF=''
while read i; do
    echo "//!$i" >> src/lib.rs;
done < README.md

cat src/lib_new.rs >> src/lib.rs
rm -f src/lib_new.rs
echo "done"

#!/bin/bash

set -e
set -x

DAY=$1
DAYXX=$(printf "%02d" "$DAY")
DAY_NAME="day${DAYXX}"

DAY_MOD="${DAY_NAME}.rs"

cp day.rs.template "src/${DAY_MOD}"
touch "data/${DAY_NAME}.txt"
echo "pub mod ${DAY_NAME};" >> src/lib.rs

what="// $DAY => aoc2023::$DAY_NAME::run,"
to="$DAY => aoc2023::$DAY_NAME::run,"
sed -i "s@$what@$to@g" src/main.rs

cargo build --release

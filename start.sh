#!/bin/bash
echo "ARCANUM"
[ ! -f .env ] && cp .env.example .env
cargo build --release
./target/release/arcanum

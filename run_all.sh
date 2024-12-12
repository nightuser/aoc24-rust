#!/bin/bash
bold="$(tput bold)"
normal="$(tput sgr0)"
cargo build --release
for day in day*; do
    echo "executing ${bold}${day}${normal}"
    hyperfine -N -w 10 --style color "\"target/release/${day}\" \"${day}/in.txt\"" 2>/dev/null
done

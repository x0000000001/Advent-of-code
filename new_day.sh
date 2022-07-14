#!/bin/bash
read year
read day
git add -A
git commit -m "day finished"
git push
cargo new "${year}/day${day}"
cp template/* "${year}/day${day}/src"
code "${year}/day${day}"

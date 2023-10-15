#!/bin/bash

num_of_nodes=$1
curtime=$2

cargo run $(pwd)/config.txt 0 $curtime & # launch leader

for ((i=1; i<$num_of_nodes; i++))
do
    cargo run $(pwd)/config.txt $i $curtime &
done

wait
exit 0 
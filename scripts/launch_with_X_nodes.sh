#!/bin/bash

num_of_nodes=$1
F=$2
curtime=$3

cargo run $(pwd)/config.txt 0 $F $curtime & # launch leader

for ((i=1; i<$num_of_nodes; i++))
do
    cargo run $(pwd)/config.txt $i $F $curtime &
done

wait
exit 0 
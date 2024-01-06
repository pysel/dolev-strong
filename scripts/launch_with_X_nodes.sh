#!/bin/bash

num_of_nodes=$1
F=$2
curtime=$3

upperbound_F=$(($num_of_nodes - 2))

if [ $num_of_nodes -lt 3 ]; then
    echo -e "\033[31mERROR:\033[0m Number of nodes should be at least 3: got $num_of_nodes, expected 3 or more" >&2
    exit 1
fi

if [ $F -gt $upperbound_F ]; then
    echo -e "\033[31mERROR:\033[0m F should not be set to more than number of nodes - 2: got $F, expected $upperbound_F or less" >&2
    exit 1
fi

mkdir $(pwd)/log
touch $(pwd)/log/node0.log

cargo run $(pwd)/config.txt 0 $F $curtime > $(pwd)/log/node0.log & # launch leader

for ((i=1; i<$num_of_nodes; i++))
do
    touch $(pwd)/log/node$i.log
    cargo run $(pwd)/config.txt $i $F $curtime > $(pwd)/log/node$i.log &
done

wait
exit 0 
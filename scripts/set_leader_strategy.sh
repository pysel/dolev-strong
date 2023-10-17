#!/bin/bash
# This script is used to set the leader strategy in config file. 
# Usage: ./set_leader_strategy.sh <strategy>
# <strategy> can be one of the following:
# 1. "leader" (corresponds to honest behavior)
# 2. "leader_null_proposal" (corresponds to malicious behavior: no proposal send out during genesis stage)

strategy=$1
valid_strategies=("leader" "leader_null_proposal")

if [[ ! " ${valid_strategies[@]} " =~ " ${strategy} " ]]; then
    echo -e "\033[31mERROR:\033[0m Invalid strategy: got $strategy, expected one of [${valid_strategies[@]}]" >&2
    exit 1
fi

awk 'NR==1 { $2="'"$strategy"'" } 1' config.txt > tmp.txt && mv tmp.txt config.txt

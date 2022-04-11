#!/usr/bin/bash


function build() {
    local user=$1
    local tag=$2
    local locate=$3
    docker build -t $user/ms-$locate:$tag $locate
}

declare -A folders
folders[0]=catalog
folders[1]=checkout
folders[2]=order
folders[3]=payment
folders[4]=products

for folder in ${folders[@]}
do
    build paulogabrieljb 0.1 $folder 
done
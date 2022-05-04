#!/usr/bin/sh


for f in $(ls *.yaml)
do
    kubectl apply -f $f
done

declare -A deployments 

deployments[0]=product-8000
deployments[1]=catalog-3000
deployments[2]=checkout-8082

for s in ${deployments[@]}
do
    name=$(echo $s | cut -d '-' -f 1)
    port=$(echo $s | cut -d '-' -f 2)
    
    kubectl expose deployment $name --port=$port --type=NodePort
done
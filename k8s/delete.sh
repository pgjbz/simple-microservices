#!/usr/bin/sh


for f in $(ls *.yaml)
do
    kubectl delete -f $f
done

#!/bin/bash

usage () {
    echo "usage: $0 <year> <day> <name>"
}

main () {
    year="$1"
    day="$2"
    name="$3"
    if [ -z "$year" ] || [ -z "$day" ] || [ -z "$name" ] ; then
        usage
        exit 1
    fi 
    if [ ! -d $year ] ; then
        mkdir $year
        echo "Be sure to add '${year}/day-*' to Cargo.toml"
    fi
    path="${year}/day-${day}"
    cp -a template $path
    cd $path
    sed -i "s/<YEAR>/${year}/g" Cargo.toml
    sed -i "s/<DAY>/${day}/g" Cargo.toml
    sed -i "s/<NAME>/${name}/g" src/main.rs
    echo "Created $path"
}

main "$@"

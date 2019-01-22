#!/bin/bash

try() {
    expected="$1"
    input="$2"
    echo -n "$input" | ./rcp
    gcc -o tmp tmp.s
    ./tmp
    actual="$?"
    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$expected expected, but got $actual"
        exit 1
    fi
}

try 0 0
try 100 100

echo OK

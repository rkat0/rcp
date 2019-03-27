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

try 0 "0;"
try 100 "100;"
try 3 "1+2;"
try 4 "5-1;"
try 10 "100-52-83+45;"
try 5 "10 -     5;"
try 10 " 100 - 52 - 83 + 45 ;"
try 2 "5 / 2;"
try 21 "10 * 3 - (5 + 8 / 2);"

echo OK

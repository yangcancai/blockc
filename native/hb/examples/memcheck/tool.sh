#!/bin/bash

check(){
	cargo run
	valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all ./target/debug/memcheck
}
help(){
	echo "sh tool.sh check -- check leak"
}
case $1 in
	check) check;;
	*) help;;
esac
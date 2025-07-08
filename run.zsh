#!/usr/bin/env zsh

source dbg.sh
source other.zsh

myfunction() {
	fdb_tracepoint myfunction
	echo "running myfunction"
}


fdb_tracepoint start
echo "Starting"
myfunction
otherfunc
fdb_tracepoint end
echo "Ending"


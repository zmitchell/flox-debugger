#!/usr/bin/env zsh

source dbg.zsh

myfunction() {
	fdb_tracepoint myfunction
	echo "running myfunction"
	otherfunc
}

otherfunc() {
	fdb_tracepoint otherfunc
	echo "running otherfunc"
	stack_trace
}

fdb_tracepoint start
echo "Starting"
myfunction
fdb_tracepoint end
echo "Ending"





















































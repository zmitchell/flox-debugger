#!/usr/bin/env bash

set -euo pipefail

source dbg.sh

myfunction() {
	fdb_tracepoint myfunction
	echo "running myfunction"
	otherfunc
}

otherfunc() {
	fdb_tracepoint otherfunc
	echo "running otherfunc"
	otherfunc2
}

otherfunc2() {
	fdb_tracepoint otherfunc2
	echo "running otherfunc2"
	otherfunc3
}

otherfunc3() {
	fdb_tracepoint otherfunc3
	echo "running otherfunc3"
}

fdb_tracepoint start
echo "Starting"
myfunction
fdb_tracepoint end
echo "Ending"









































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
}

fdb_tracepoint start
echo "Starting"
myfunction
fdb_tracepoint end
echo "Ending"









































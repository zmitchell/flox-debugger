#!/usr/bin/env fish

source dbg.fish

function myfunction
    fdb_tracepoint myfunction
    echo "running myfunction"
    otherfunc
end

function otherfunc
    fdb_tracepoint otherfunc
    echo "running otherfunc"
end

fdb_tracepoint start
echo Starting
myfunction
fdb_tracepoint end
echo Ending



































































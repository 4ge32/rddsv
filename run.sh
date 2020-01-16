#!/bin/sh

dir=res

cargo run > res/p_cas.dot
dot -Tpdf -o $dir/p_cas.pdf $dir/p_cas.dot

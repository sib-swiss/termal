#!/usr/bin/expect -f
spawn termal ../data/big.msa
expect "l"  ;# or a suitable prompt if any
#send "Q\r"

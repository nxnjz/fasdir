#!/usr/bin/bash

while true
do
  read -p "enter url scheme: " scheme
  read -p "enter host: " host
  fasdir -w ~/wl/web-content/nxnjzhttp.txt --random-user-agent -t 60 -x ,/,.txt,.json,.html,.xml,.js,.php,.asp,.aspx,.htm,.jsp -o fasdirbulk.tmp -O -s 1-403,405-999 -u $scheme$host
  mv fasdirbulk.tmp $host.fasdir.tmp
  echo output at $host.fasdir.tmp
done

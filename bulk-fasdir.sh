#!/usr/bin/bash

while true
do
  read -p "enter url scheme: " scheme
  read -p "enter host: " host
  read -p "enter base path: " path
  fasdir -w ~/wl/web-content/nxnjzhttp.txt --random-user-agent -t 60 -x ,/,.txt,.json,.html,.xml,.js,.php,.asp,.aspx,.htm,.jsp -o $host.fasdir.tmp -s 1-403,405-999 -u $scheme$host$path
  output = $host.fasdir$RANDOM.txt
  mv $host.fasdir.tmp $output
  echo output at $output
done

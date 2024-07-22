#!/bin/bash
cat words.txt| tr ' ' '\n'|grep -v -E '^$'|sort|uniq -c|awk '{print $2 " " $1}'|sort -rn -k2

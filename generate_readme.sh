#!/bin/bash
replacer=$(cat target/replacer.txt)
key=${replacer%:*}
value=${replacer#*:}
sed "s/$key/$value/" TEMPLATE.md > README.md
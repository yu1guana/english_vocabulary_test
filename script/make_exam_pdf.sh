#!/bin/bash

readonly NOW_DIR=$(pwd)

for f in $@; do
  if [ ! -f $f ]; then
    echo "Error: $f does not exist." 1>&2
    continue
  fi
  tex_file=$(basename $f | sed "s/\(.*\)\.toml/exam_of_\1.tex/")
  cd $(dirname $f)
  latexmk $tex_file
  latexmk -c $tex_file
  cd $NOW_DIR
done

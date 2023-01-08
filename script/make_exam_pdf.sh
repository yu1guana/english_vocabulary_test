#!/bin/bash

readonly NOW_DIR=$(pwd)

if [ $# -eq 0 ]; then
  echo "Error: arguments are necessary." 1>&2
  exit 1
fi

for f in $@; do
  if [ ! -f $f ]; then
    echo "Error: $f does not exist." 1>&2
    continue
  fi
  exam_tex_file=$(basename $f | sed "s/\(.*\)\.toml/exam_of_\1.tex/")
  answer_tex_file=$(basename $f | sed "s/\(.*\)\.toml/answer_of_\1.tex/")
  cd $(dirname $f)
  latexmk $exam_tex_file
  latexmk -c $exam_tex_file
  latexmk $answer_tex_file
  latexmk -c $answer_tex_file
  cd $NOW_DIR
done

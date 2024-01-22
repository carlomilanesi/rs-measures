#!/bin/sh

cargo b --release

FILELIST=`ls rs-measures/src/define_*.rs rs-measures/src/inner/*.rs | grep -v "/mod.rs$"`

for FILEPATH in $FILELIST
do
    target/release/format-measure-n-d comment $FILEPATH
done

cargo fmt

for FILEPATH in $FILELIST
do
    target/release/format-measure-n-d uncomment $FILEPATH
done

cargo fmt --check

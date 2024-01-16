#!/bin/sh

cargo b --release

for ORIG_FILEPATH in rs-measures/src/measure*d.rs
do
    TEMP_FILEPATH=${ORIG_FILEPATH}_
    target/release/format-measure-n-d comment <$ORIG_FILEPATH >$TEMP_FILEPATH
    mv -f $TEMP_FILEPATH $ORIG_FILEPATH
done

cargo fmt

for ORIG_FILEPATH in rs-measures/src/measure*d.rs
do
    TEMP_FILEPATH=${ORIG_FILEPATH}_
    target/release/format-measure-n-d uncomment <$ORIG_FILEPATH >$TEMP_FILEPATH
    mv -f $TEMP_FILEPATH $ORIG_FILEPATH
done

cargo fmt --check

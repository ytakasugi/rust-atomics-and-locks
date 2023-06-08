#!/bin/bash

TARGET_FILE=`basename $1 .rs`

cargo run --example ${TARGET_FILE}

#!/bin/bash

## Prerequisites: `sudo npm install -g json-fmt`

TESTDENALI_SCEN_FILES=$(find . -name "*.scen.json")
TESTDENALI_STEP_FILES=$(find . -name "*.step.json")
TESTDENALI_STEPS_FILES=$(find . -name "*.steps.json")
TESTDENALI_ALL_FILES="$TESTDENALI_SCEN_FILES $TESTDENALI_STEP_FILES $TESTDENALI_STEPS_FILES"

TEMP_FILE=testdenali-fmt-temp.scen.json
for TESTDENALI_FILE in $TESTDENALI_ALL_FILES
do
    echo $TESTDENALI_FILE
    json-fmt $TESTDENALI_FILE --indent "    " --prettify --output $TEMP_FILE || exit 1
    echo >> $TEMP_FILE # adds missing newline
    mv $TEMP_FILE $TESTDENALI_FILE
done

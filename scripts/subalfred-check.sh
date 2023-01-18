#!/bin/bash

# Make sure this file is clean before running dir iteration
> list

for dir in $(ls pallets); do 
    if [[ $dir == "mock" ]]; then 
        continue; 
    fi;

    echo pallets/$dir >> list
done

for dir in $(ls runtime); do 
    if [[ $dir == "mock" ]]; then 
        continue; 
    fi;

    echo runtime/$dir >> list
done

ERRORS=false

for dir in $(cat list); do 
    echo 
    RESULT=$(subalfred check features $dir)
    CHECK_RESULT=$? # 0 if it's good, anything else is bad 

    echo $RESULT | grep '`std`' > /dev/null
    GREP_RESULT=$? # 0 if it's bad, 1 if it's good


    # If there are no errors in subalfred check, then we're good
    if [[ $CHECK_RESULT == 0 ]]; then
        echo "✅ $dir"

    # If there are std features, but no errors, then we're yellow
    elif [[ $GREP_RESULT == 1 && $CHECK_RESULT != 0 ]]; then
        echo "🟡 $dir"
        echo -e "$RESULT"

    # If there are std errors, then we're red
    else
        echo "❌ $dir"
        echo -e "$RESULT"
        ERRORS=true
    fi
done

if [[ $ERRORS == true ]]; then
    exit 1
fi


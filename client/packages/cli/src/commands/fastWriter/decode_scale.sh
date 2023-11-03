#!/bin/bash

# Function to decode SCALE-encoded array of 4-byte ASCII characters
decode_scale_ascii() {
    local encoded_string="$1"
    # Remove the leading 0x and the first 2 bytes which represent the length
    local stripped_hex=${encoded_string:4}
    local decoded_array=()

    # Each ASCII character is represented by 2 hex characters
    while [ -n "$stripped_hex" ]; do
        # Extract 4 bytes (8 hex characters)
        local bytes="${stripped_hex:0:8}"
        stripped_hex="${stripped_hex:8}"

#        print "$stripped_hex"
        # Convert hex to ASCII, append to the decoded array
        local ascii=$(echo "$bytes" | xxd -r -p)
        # Check if ASCII is a printable character and if not fall back to bytes as decoded array element (difference between "roco" or "0x03030303")
        if [[ "$ascii" =~ ^[[:print:]]+$ ]]; then
            decoded_array+=("$ascii")
        else
            decoded_array+=("$bytes")
        fi
    done

    # Output the decoded array
    printf "[ %s ]" "${decoded_array[@]}"
}

# Function to decode SCALE-encoded array of u32 values
decode_scale_u32() {
    local encoded_string="$1"
    # Remove the leading 0x and the first 2 bytes which represent the length
    local stripped_hex=${encoded_string:4}
    local decoded_array=()

    # Each u32 value is 4 bytes
    while [ -n "$stripped_hex" ]; do
        # Extract 4 bytes (8 hex characters), and reverse the byte order for little-endian
        local bytes="${stripped_hex:6:2}${stripped_hex:4:2}${stripped_hex:2:2}${stripped_hex:0:2}"
        stripped_hex="${stripped_hex:8}"

        # Convert hex to decimal
        local decimal=$((16#$bytes))
        decoded_array+=("$decimal")
    done

    # Output the decoded array
    printf "[ %s ]" "${decoded_array[@]}"
}

# Example usage:
# Suppose we received the hex strings from curl commands, now we want to decode them
targets_hex="0x1403030303726f636f7365706c6b75736d70646f74"
assets_hex="0x0ce9030000d0070000b80b0000"

# Decode and display the results
echo "Decoded Targets:"
decode_scale_ascii "$targets_hex"
echo ""
echo "Decoded Assets:"
decode_scale_u32 "$assets_hex"

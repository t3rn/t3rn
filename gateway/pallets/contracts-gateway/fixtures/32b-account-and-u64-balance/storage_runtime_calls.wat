(module
	(import "seal0" "seal_return" (func $seal_return(param i32 i32 i32)))
	(import "seal0" "seal_call" (func $seal_call (param i32 i32 i64 i32 i32 i32 i32 i32 i32) (result i32)))
    (import "seal0" "seal_input" (func $seal_input (param i32 i32)))
	(import "env" "memory" (memory 1 1))

    ;; [32, 64) bytes for a module name
    (data (i32.const 32) "Weights")

    ;; [64, 96) bytes for a function name
    (data (i32.const 64) "store_value")

    ;; [96, 100) buffer where input is copied (u32 value to put into storage)

    ;; [100, 104) size of the input buffer
    (data (i32.const 100) "\04")

    (func $assert (param i32)
        (block $ok
            (br_if $ok
                (get_local 0)
            )
            (unreachable)
        )
    )

	(func (export "deploy"))

	(func (export "call")
        (local $address_length i32)
        (local $balance_length i32)
        (set_local $address_length (i32.const 32)) ;; 32 bytes of account length for demo-runtime runtime
        (set_local $balance_length (i32.const 8)) ;; u128 for demo-runtime runtime

        (call $seal_input (i32.const 96) (i32.const 100))

        (call $seal_call
            (i32.const 0)	;; Pointer to destination address
            (get_local $address_length)	;; Length of destination address
            (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
            (i32.const 0)	;; Pointer to the buffer with value to transfer
            (get_local $balance_length)	;; Length of the buffer with value to transfer
            (i32.const 32)	;; Pointer to input data buffer address (NOTE THAT FIRST 64B RESERVER FOR FN + MOD NAMES)
            (i32.const 4)	;; Length of input data buffer
            (i32.const 4294967295) ;; Pointer to output data buffer address
            (i32.const 0) ;;Length of output data buffer
        )
        (call $seal_return
            (i32.const 0)
            (i32.const 0)
            (i32.const 0)
        )
		(unreachable)
	)
)

(module
	(import "seal0" "seal_return" (func $seal_return(param i32 i32 i32)))
	(import "seal0" "seal_call" (func $seal_call (param i32 i32 i64 i32 i32 i32 i32 i32 i32) (result i32)))
	(import "env" "memory" (memory 1 1))

    ;; [32, 64) bytes for a module name
    (data (i32.const 40) "Flipper")

    ;; [64, 96) bytes for a function name
    (data (i32.const 72) "flip")

	(func (export "deploy"))

	(func (export "call")
        (call $seal_call
            (i32.const 8)	;; Pointer to destination address
            (i32.const 32)	;; Length of destination address
            (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
            (i32.const 0)	;; Pointer to the buffer with value to transfer
            (i32.const 8)	;; Length of the buffer with value to transfer
            (i32.const 40)	;; Pointer to input data buffer address
            (i32.const 0)	;; Length of input data buffer
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

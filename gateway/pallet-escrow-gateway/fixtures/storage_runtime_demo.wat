(module
	(import "seal0" "seal_return" (func $seal_return(param i32 i32 i32)))
	(import "seal0" "seal_call" (func $seal_call (param i32 i32 i64 i32 i32 i32 i32 i32 i32) (result i32)))
    (import "seal0" "seal_input" (func $seal_input (param i32 i32)))
    (import "seal0" "seal_deposit_event" (func $seal_deposit_event (param i32 i32 i32 i32)))
	(import "env" "memory" (memory 1 1))

    ;; [32, 64) bytes for a module name
    (data (i32.const 32) "Weights")
    ;; [64, 96) bytes for a function name
    (data (i32.const 64) "store_value")

    ;; [96, 100) buffer where input is copied (u32 value to put into storage)

    ;; [100, 104) size of the input buffer
    (data (i32.const 100) "\04")

    ;; [104, 136) bytes for a module name
    (data (i32.const 104) "Weights")
    ;; [136, 168) bytes for a 2nd function name
    (data (i32.const 136) "double")

    ;; [168, 172) buffer where input is copied (u32 value to put into storage)

    ;; [172, 176) size of the input buffer
    (data (i32.const 172) "\04")

    ;; [176, 208) bytes for a module name
    (data (i32.const 176) "Weights")
    ;; [208, 240) bytes for a 3rd function name
    (data (i32.const 208) "complex_calculations")

    ;; [240, 248) buffer where input is copied (x: u32 , y: u32 values for complex calculation parameters)
    (data (i32.const 240) "\06") ;; x: u32
    (data (i32.const 244) "\07") ;; y: u32

    ;; [244, 248) size of the input buffer
    (data (i32.const 248) "\08")


    ;; Set topics for deposit_event after calls are made
    (data (i32.const 512) "\04\83\116\111\114\97\103\101\32\45\32\115\101\116\95\118\97\108\117\101") ;; Storage - set_value as utf8 bytes

    (data (i32.const 576) "\04\83\116\111\114\97\103\101\32\45\32\100\111\117\98\108\101") ;; Storage - double" as utf8 bytes

    (data (i32.const 640) "\04\83\116\111\114\97\103\101\32\45\32\99\111\109\112\108\101\120\95\99\97\108\99\117\108\97\116\105\111\110\115") ;; Storage - complex_calculations") as utf8 bytes

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
        (call $seal_input (i32.const 96) (i32.const 100))

        (call $seal_call
            (i32.const 16)	;; Pointer to destination address
            (i32.const 8)	;; Length of destination address
            (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
            (i32.const 0)	;; Pointer to the buffer with value to transfer
            (i32.const 8)	;; Length of the buffer with value to transfer
            (i32.const 32)	;; Pointer to input data buffer address (NOTE THAT FIRST 64B RESERVER FOR FN + MOD NAMES)
            (i32.const 4)	;; Length of input data buffer
            (i32.const 4294967295) ;; Pointer to output data buffer address
            (i32.const 0) ;;Length of output data buffer
        )

        (call $seal_deposit_event
            (i32.const 512) ;; The topics buffer
            (i32.const 64) ;; The topics buffer's length
            (i32.const 0) ;; The data buffer
            (i32.const 0) ;; The data buffer's length
        )

        ;; Copy out the input given by a user to to input for the next "double" call
        (i32.store
            (i32.const 168)
            (i32.load (i32.const 96))
        )

        (call $seal_call
            (i32.const 16)	;; Pointer to destination address
            (i32.const 8)	;; Length of destination address
            (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
            (i32.const 0)	;; Pointer to the buffer with value to transfer
            (i32.const 8)	;; Length of the buffer with value to transfer
            (i32.const 104)	;; Pointer to input data buffer address (NOTE THAT FIRST 64B RESERVER FOR FN + MOD NAMES)
            (i32.const 4)	;; Length of input data buffer
            (i32.const 4294967295) ;; Pointer to output data buffer address
            (i32.const 0) ;;Length of output data buffer
        )

        (call $seal_deposit_event
            (i32.const 576) ;; The topics buffer
            (i32.const 64) ;; The topics buffer's length
            (i32.const 0) ;; The data buffer
            (i32.const 0) ;; The data buffer's length
        )

        (call $seal_call
            (i32.const 16)	;; Pointer to destination address
            (i32.const 8)	;; Length of destination address
            (i64.const 0)	;; How much gas to devote for the execution. 0 = all.
            (i32.const 0)	;; Pointer to the buffer with value to transfer
            (i32.const 8)	;; Length of the buffer with value to transfer
            (i32.const 176)	;; Pointer to input data buffer address (NOTE THAT FIRST 64B RESERVER FOR FN + MOD NAMES)
            (i32.const 8)	;; Length of input data buffer
            (i32.const 4294967295) ;; Pointer to output data buffer address
            (i32.const 0) ;;Length of output data buffer
        )

        (call $seal_deposit_event
            (i32.const 640) ;; The topics buffer
            (i32.const 64) ;; The topics buffer's length
            (i32.const 0) ;; The data buffer
            (i32.const 0) ;; The data buffer's length
        )

        (call $seal_return
            (i32.const 0)
            (i32.const 0)
            (i32.const 0)
        )
		(unreachable)
	)
)

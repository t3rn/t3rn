;; This transfers 100 balance to the zero account and copies the return code
;; of this transfer to the output buffer.
(module
	(import "seal0" "seal_transfer" (func $seal_transfer (param i32 i32 i32 i32) (result i32)))
	(import "seal0" "seal_return" (func $seal_return (param i32 i32 i32)))
	(import "env" "memory" (memory 1 1))

	;; [0, 8) zero-adress
	(data (i32.const 0) "\00\00\00\00\00\00\00\00")

	;; [8, 16) 100 balance
	(data (i32.const 8) "\64\00\00\00\00\00\00\00")

	;; [16, 20) here we store the return code of the transfer

	(func (export "deploy"))

	(func (export "call")
		(i32.store
			(i32.const 16)
			(call $seal_transfer
				(i32.const 0) ;; ptr to destination address
				(i32.const 8) ;; length of destination address
				(i32.const 8) ;; ptr to value to transfer
				(i32.const 8) ;; length of value to transfer
			)
		)
		;; exit with success and take transfer return code to the output buffer
		(call $seal_return (i32.const 0) (i32.const 16) (i32.const 4))
	)
)

(module
  (import "env" "log" (func $log (param i32 i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "Hello from Wasm via host import!")
  (func (export "run")
    (call $log (i32.const 0) (i32.const 33)))
)

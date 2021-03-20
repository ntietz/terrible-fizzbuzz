This terrible fizzbuzz implementation uses no conditionals! We employ loop unrolling to make this happen, as well as making an explicit mapping of the modulus to the corresponding function.

You can inspect the generated assembly code:
```
cargo rustc --release -- --emit asm
```

Upon inspection, you'll note there are no conditional jumps or otherwise conditional instructions.

---
marp: true
paginate: true
color: #ffff
backgroundColor: #2A2A2A
header: '![width:100px height:100px](./img/logo.png)'
footer: "**19/01/2023 - AnthonyF et ZyadS**"
---
<style>
section {
  font-family: 'Century Gothic', serif !important;
}
</style>
<!-- _class: invert -->

# Oxidize-OS Presentation <!-- fit -->

---
<!-- _class: invert -->

# Sommaire

- Rust programming language
- Linux Kernel
- Oxidize-OS
- Environment setup
- Simple code explanation
- Next steps

---
<!-- _class: invert -->

# Rust programming language

---
<!-- _class: invert -->

# Linux Kernel

---
<!-- _class: invert -->

# Environment setup

---
<!-- _class: invert -->

# Simple code explanation

---
<!-- _class: invert -->

# Disabling the standard library

- No standard functions (macros, multithresding, I/O, etc.)
- No C standard library `libc`

```Rust
#![no_std]  
```

---
<!-- _class: invert -->

## Rust attributes

- Outer
`#[foo]`

- Inner
`#![foo]`

```Rust
#[foo]
struct Foo;

fn foo() {
    #![foo]
}
```

---
<!-- _class: invert -->

# Panic handler

```Rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

---
<!-- _class: invert -->

# Entry point

```Rust
#[no_mangle]
pub extern "C" fn _start() -> ! { 
    loop{}
}
```

---
<!-- _class: invert -->

# Simple print

```Rust
static STRING: &[u8] = b"Oxidize OS";
let vga_buffer = 0xb8000 as *mut u8;

for (i, &byte) in STRING.iter().enumerate() {
    unsafe {
        *vga_buffer.offset(i as isize * 2) = byte;
        *vga_buffer.offset(i as isize * 2 + 1) = 0xf;
    }
}
```

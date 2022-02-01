# references
- [blog os](https://os.phil-opp.com/freestanding-rust-binary/)
- [os in C](https://github.com/cfenollosa/os-tutorial)
- [x86 registers](https://en.wikibooks.org/wiki/X86_Assembly/16,_32,_and_64_Bits)
- [VGA buffer](https://web.archive.org/web/20150816220334/http://www.eyetap.org/cyborgs/manuals/soft_vga.pdf)
- [maybe usable?](https://github.com/alilee/rust-osdev-jumpstart)

# running
- only tested on mac
```shell
$ # install qemu
$ brew install qemu # mac with homebrew
$ sudo pacman -S qemu # arch

$ rustup update nightly # maybe

$ make
```

# Plans
- do bootloader part in assembly
- video
- file system
- elf?
- networking?
- shell?



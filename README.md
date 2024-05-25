# multiatch-relocate-example

**UNFINISHED YET**

Make a single uprobe.bpf.o able to work on both x86_64 and aarch64

Way:
- Build that object file using vmlinux header of x86, so it would be able to work on x86
- Use a script to relocate that file, replace access to `struct pt_regs` of x86 to the offset of aarch64. aarch64 and x86_64 has the same register size, so we only need to modify the offset

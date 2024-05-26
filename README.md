# multiatch-relocate-example

**UNFINISHED YET**

Make a single uprobe.bpf.o able to work on both x86_64 and aarch64

Way:
- Build that object file using vmlinux header of x86, so it would be able to work on x86
- Patch the object file bases on CORE relocation records in .BTF.ext, manually patch access to struct pt_regs with offsets to aarch64, then remove .BTF.ext section.

## References
- https://docs.kernel.org/bpf/llvm_reloc.html#co-re-relocations

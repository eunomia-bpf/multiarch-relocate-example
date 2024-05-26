# multiatch-relocate-example

Make a single uprobe.bpf.o able to work on both x86_64 and aarch64

Way:
- Build that object file using vmlinux header of x86, so it would be able to work on x86
- Patch the object file bases on CORE relocation records in .BTF.ext, manually patch access to struct pt_regs with offsets to aarch64, then remove .BTF.ext section.

## How to use?
- Run `make -C ./uprobe -j8 && cp ./uprobe/.output/uprobe.bpf.o . && cp ./uprobe/target .` to get a bpf object file which is able to run on x86
- Run `make loader` and `./loader uprobe.bpf.o`, and run `./target` in another terminal. See `/sys/kernel/tracing/trace_pipe` for output. This can prove that the ebpf program is able to run on x86
- Run `cd relocate && cargo run -- ../uprobe.bpf.o` to generate `uprobe_new.bpf.o`. This step requires Rust.

On aarch64 machine (Debian12 ARM64 has been tested): 
- Run `make -C ./uprobe target && cp ./uprobe/target .`
- Run `make loader`. This steps requires libbpf
- Copy that `uprobe_new.bpf.o` to the project root
- Run `./loader uprobe_new.bpf.o`, and `./target` in another terminal
- See `/sys/kernel/tracing/trace_pipe` for output

## References
- https://docs.kernel.org/bpf/llvm_reloc.html#co-re-relocations

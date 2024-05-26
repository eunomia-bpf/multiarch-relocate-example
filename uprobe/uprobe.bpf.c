#include <linux/bpf.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>


char LICENSE[] SEC("license") = "GPL";

struct pt_regs {
  long unsigned int r15;
  long unsigned int r14;
  long unsigned int r13;
  long unsigned int r12;
  long unsigned int bp;
  long unsigned int bx;
  long unsigned int r11;
  long unsigned int r10;
  long unsigned int r9;
  long unsigned int r8;
  long unsigned int ax;
  long unsigned int cx;
  long unsigned int dx;
  long unsigned int si;
  long unsigned int di;
  long unsigned int orig_ax;
  long unsigned int ip;
  long unsigned int cs;
  long unsigned int flags;
  long unsigned int sp;
  long unsigned int ss;
};

SEC("uprobe/./target:uprobe_add")
int BPF_KPROBE(uprobe_add) {
  long a = BPF_CORE_READ(ctx, di);
  long b = BPF_CORE_READ(ctx, si);
  bpf_printk("uprobed_add ENTRY: a = %d, b = %d", a, b);
  return 0;
}

SEC("uprobe/./target:uprobe_sub")
int BPF_KPROBE(uprobe_sub) {
  long a = BPF_CORE_READ(ctx, di);
  long b = BPF_CORE_READ(ctx, si);
  bpf_printk("uprobed_sub ENTRY: a = %d, b = %d", a, b);
  return 0;
}

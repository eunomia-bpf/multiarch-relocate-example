#include <bpf/bpf.h>
#include <bpf/libbpf.h>
#include <cassert>
#include <chrono>
#include <iostream>
#include <ostream>
#include <thread>
int main(int argc, const char **argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " [uprobe_new.bpf.o]" << std::endl;
    return 1;
  }
  auto object = bpf_object__open_file(argv[1], nullptr);
  assert(object);
  int err;
  err = bpf_object__load(object);
  assert(err >= 0);
  bpf_program *curr_prog = nullptr;
  bpf_object__for_each_program(curr_prog, object) {
    auto ret = bpf_program__attach(curr_prog);
    assert(ret);
  }
  std::cout << "Attached.." << std::endl;
  while (true) {
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
  return 0;
}

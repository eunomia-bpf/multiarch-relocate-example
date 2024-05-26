#define _GNU_SOURCE
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int uprobe_add(int a, int b) { return a + b; }

int uprobe_sub(int a, int b) { return a - b; }

int main() {
  srand(time(NULL));
  while (1) {
    int a = rand() & 0xff;
    int b = rand() & 0xff;
    printf("%d %d\n", uprobe_add(a, b), uprobe_sub(a, b));
    sleep(2);
  }
}

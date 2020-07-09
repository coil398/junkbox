#include <stdlib.h>
#include <stdio.h>

void take_ownership(int *i, void (*dtor)(int *)) {
  printf("got %d\n", *i);
  dtor(i);
}

int *make_memory() {
  int *i;

  i = malloc(sizeof(int));
  *i = 2;

  return i;
}

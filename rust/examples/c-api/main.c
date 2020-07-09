#include <stdio.h>
#include <stdint.h>

struct point {
  int x;
  int y;
};

double dist(struct point *, struct point *);

int main() {
  struct point p1 = {1, 0}, p2 = {0, 1};
  double ret;

  ret = dist(&p1, &p2);

  printf("%f\n", ret);
}

#include <stdio.h>

int sum(int x, int y) { return x + y; }

int main(int argc, char const *argv[]) {
  int x = 0;
  int y = 0;

  scanf("%d %d\n", &x, &y);

  printf("You entered: %d\n", sum(x, y));

  return 0;
}

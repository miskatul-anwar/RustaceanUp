#include <algorithm>
#include <bits/stdc++.h>
#include <iostream>

#define fast_io                                                                \
  std::ios::sync_with_stdio(false);                                            \
  std::cin.tie(nullptr);                                                       \
  std::cout.tie(nullptr);
using namespace std;

int main() {
  fast_io;
  int t;
  cin >> t;

  int cap = 0;
  int max_cap = -1;

  while (t--) {
    int a;
    int b;
    cin >> a >> b;

    cap -= a, cap += b;
    max_cap = max(max_cap, cap);
  }

  cout << max_cap << '\n';
  return 0;
}

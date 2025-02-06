#include <iostream>
#include <string>
using namespace std;

void replace(string &origin, string &target, string &rep) {
  int n = origin.length();
  for (int i = 0; i < n; i++) {
    if (origin[i] == target[0]) {
      bool match = true;
      for (int j = 0; j < target.size(); j++) {
        if (target[j] != origin[i + j]) {
          match = false;
          break;
        }
      }
      if (match) {
        for (int j = 0; j < target.size(); j++) {
          origin[i + j] = rep[j];
        }
      }
    }
  }
}

int main(int argc, char *argv[]) {
  string origin, target, rep;
  cin >> origin >> target >> rep;

  replace(origin, target, rep);
  std::cout << origin << '\n';
  return 0;
}

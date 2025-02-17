#include <bits/stdc++.h>
using namespace std;

int min_ope(const vector<int>&a,int n){
  bool equal=true;
  for (size_t i = 1; i < n; i++) {
     if (a[i]!=a[0]) {
      equal=false; 
      break;
     }
  }

  if (equal) return 0;

  bool non_inc = true;
  for (size_t i = 1; i < n; i++) {
    if (a[i]>a[i-1]) {
     non_inc=false; 
      break;
    }
  }
  
  if (non_inc) return 1;
  return 2;
}

int main (int argc, char *argv[]) {
  int t;
  cin >> t;

  while (t--) {
   int n;
   cin >> n;
    vector<int>a(n);
    for (size_t i = 0; i < n; i++) {
     cin >> a[i]; 
    }

    cout << min_ope(a,n)<<'\n';
  }
  return 0;
}

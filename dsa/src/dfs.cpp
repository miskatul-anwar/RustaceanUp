#include <bits/stdc++.h>
using namespace std;
dfs(int vertex, vector<vector<int>> graph, vector<bool> visited) {
  visited[vertex] = true;

  for (auto child : graph) {
    if (!visited[child]) {
      dfs(child, graph, visited);
    }
  }
}

int main() { return 0; }
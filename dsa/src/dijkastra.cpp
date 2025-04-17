#include <bits/stdc++.h>
using namespace std;
typedef long long ll;

const ll INF = 1e18;
int n, m;
vector<pair<ll, ll>> adj[100005];
ll d[100005], p[100005];

void print_path(ll i) {
  if (i != 1)
    print_path(p[i]);
  cout << i << ' ';
}

int main() {
  cin >> n >> m;

  for (int i = 0; i < m; ++i) {
    ll a, b, w;
    cin >> a >> b >> w;
    adj[a].push_back({b, w});
    adj[b].push_back({a, w});
  }

  for (int i = 1; i <= n; ++i)
    d[i] = INF;
  d[1] = 0;

  priority_queue<pair<ll, ll>, vector<pair<ll, ll>>, greater<>> q;
  q.push({0, 1});

  while (!q.empty()) {
    auto [dist_u, u] = q.top();
    q.pop();

    if (dist_u > d[u])
      continue;

    for (auto [v, w] : adj[u]) {
      if (d[u] + w < d[v]) {
        d[v] = d[u] + w;
        p[v] = u;
        q.push({d[v], v});
      }
    }
  }

  if (d[n] == INF) {
    cout << -1 << '\n';
  } else {
    print_path(n);
    cout << '\n';
  }

  return 0;
}

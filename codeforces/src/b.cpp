#include <bits/stdc++.h>

using namespace std;

void solve()
{
    int t;
    cin >> t;
    while (t--)
    {
        int n, s1, s2, m1, m2;
        cin >> n >> s1 >> s2 >> m1;
        vector<vector<int>> g1(n + 1);

        for (int i = 0; i < m1; ++i)
        {
            int u, v;
            cin >> u >> v;
            g1[u].push_back(v);
            g1[v].push_back(u);
        }

        cin >> m2;

        vector<vector<int>> g2(n + 1);

        for (int i = 0; i < m2; i++)
        {
            int u, v;
            cin >> u >> v;
            g2[u].push_back(v);
            g2[v].push_back(u);
        }

        vector<bool> good(n + 1, false);

        for (int v = 1; v <= n; v++)
        {
            set<int> s(g2[v].begin(), g2[v].end());
            for (int nb : g1[v])
            {
                if (s.count(nb))
                {
                    good[v] = true;
                    break;
                }
            }
        }
        const long long INF = LLONG_MAX;
        vector<vector<long long>> dist(n + 1, vector<long long>(n + 1, INF));

        dist[s1][s2] = 0;
    }

    priority_queue<tuple<long long, int, int>, vector<tuple<long long, int, int>>, greater<>> pq;

    pq.push(0, s1, s2);
    long long ans = -1;

    while (!pq.empty())
    {
        auto [d, u, v] = pq.top();
        if (d != dist[u][v])
        {
            continue;
        }
        if (u == v && good[u])
        {
            ans = d;
            break;
        }

        for (int nu : g1[u])
        {
            for (int nv : g2[v])
            {
                long long nd = d + abs(nu - nv);
                if (nd < dist[nu][nv])
                {
                    dist[nu][nv] = nd;
                    pq.push({nd, nu, nv});
                }
            }
        }
    }
    cout << ans << endl;
}
int main()
{
    solve();
    return 0;
}
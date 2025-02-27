#include <bits/stdc++.h>

#define FAST_IO                       \
    std::ios::sync_with_stdio(false); \
    std::cin.tie(nullptr);

void solve()
{
    int num_operations, target_sum, max_increment;
    std::cin >> num_operations >> target_sum >> max_increment;

    if (target_sum < -num_operations * max_increment || target_sum > num_operations * max_increment)
    {
        std::cout << -1 << "\n";
        return;
    }

    if (target_sum == 0)
    {
        std::cout << 0 << "\n";
        return;
    }

    int min_steps = (std::abs(target_sum) + max_increment - 1) / max_increment; // Ceiling of target_sum / max_increment

    if (min_steps > num_operations)
    {
        std::cout << -1 << "\n";
    }
    else
    {
        std::cout << min_steps << "\n";
    }
}

int main()
{
    FAST_IO;

    int test_cases;
    std::cin >> test_cases;

    while (test_cases--)
    {
        solve();
    }

    return 0;
}

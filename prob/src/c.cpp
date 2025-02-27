#include <bits/stdc++.h>

#define FAST_IO                       \
    std::ios::sync_with_stdio(false); \
    std::cin.tie(nullptr);

unsigned int compute_mask(unsigned int value)
{
    if (value == 0 || value == 1)
        return 0;

    unsigned int reduced_value = value - 1;
    int highest_bit_position = 31 - __builtin_clz(reduced_value);

    if (value == (1u << highest_bit_position))
    {
        return (1u << highest_bit_position) - 1;
    }
    else
    {
        return (1u << (highest_bit_position + 1)) - 1;
    }
}

void solve()
{
    int sequence_length;
    unsigned int target_value;
    std::cin >> sequence_length >> target_value;

    int optimal_m = 0;

    for (int current_m = 0; current_m <= sequence_length; current_m++)
    {
        if (current_m == 0)
        {
            optimal_m = 0;
        }
        else
        {
            unsigned int base_mask = compute_mask(current_m);

            if ((base_mask & (~target_value)) != 0)
                continue;

            int required_length = (base_mask == target_value) ? current_m : current_m + 1;

            if (required_length <= sequence_length)
            {
                optimal_m = current_m;
            }
        }
    }

    std::vector<unsigned int> distinct_values;

    if (optimal_m == 0)
    {
        distinct_values.push_back(target_value);
    }
    else
    {
        for (int i = 0; i < optimal_m; i++)
        {
            distinct_values.push_back(i);
        }

        unsigned int base_mask = compute_mask(optimal_m);

        if (base_mask != target_value)
        {
            unsigned int additional_value = target_value & (~base_mask);
            distinct_values.push_back(additional_value);
        }
    }

    std::vector<unsigned int> result_sequence(distinct_values.begin(), distinct_values.end());

    unsigned int fill_value = (optimal_m > 0) ? 0u : target_value;

    while (result_sequence.size() < static_cast<size_t>(sequence_length))
    {
        result_sequence.push_back(fill_value);
    }

    for (auto number : result_sequence)
    {
        std::cout << number << " ";
    }

    std::cout << "\n";
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

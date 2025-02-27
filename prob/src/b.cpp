#include <bits/stdc++.h>

#define FAST_IO                            \
    std::ios_base::sync_with_stdio(false); \
    std::cin.tie(nullptr);

int main()
{
    FAST_IO;

    int test_cases;
    std::cin >> test_cases;

    while (test_cases--)
    {
        int string_length;
        std::cin >> string_length;

        std::string input_string;
        std::cin >> input_string;

        int count_minus = 0, count_other = 0;

        for (char character : input_string)
        {
            if (character == '-')
                count_minus++;
            else
                count_other++;
        }

        if (count_minus < 2 || count_other == 0)
        {
            std::cout << 0 << '\n';
        }
        else
        {
            std::cout << (1LL * count_minus * count_minus / 4) * count_other << '\n';
        }
    }

    return 0;
}

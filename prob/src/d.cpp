#include <iostream>
#include <vector>

#define FAST_IO                       \
    std::ios::sync_with_stdio(false); \
    std::cin.tie(nullptr);

int main()
{
    FAST_IO;

    int test_cases;
    std::cin >> test_cases;

    while (test_cases--)
    {
        int array_size;
        std::cin >> array_size;

        std::vector<int> values(array_size);

        for (int i = 0; i < array_size; i++)
        {
            std::cin >> values[i];
        }

        int optimal_left = 0, optimal_right = 0;
        int optimal_difference = 0;

        for (int left = 0; left < array_size; left++)
        {
            int count_greater = 0, count_less = 0;

            for (int right = left + 1; right < array_size; right++)
            {
                if (values[right] > values[left])
                    count_greater++;
                else if (values[right] < values[left])
                    count_less++;

                int difference = count_greater - count_less;

                if (difference < optimal_difference)
                {
                    optimal_difference = difference;
                    optimal_left = left;
                    optimal_right = right;
                }
            }
        }

        std::cout << optimal_left + 1 << " " << optimal_right + 1 << "\n";
    }

    return 0;
}

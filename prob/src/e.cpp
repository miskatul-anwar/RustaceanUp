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
        int target;
        std::cin >> target;

        if (target == 0)
        {
            std::cout << 1 << "\n";
            std::cout << "0 0\n";
            continue;
        }

        std::vector<int> group_sizes;
        int remaining = target;

        while (remaining > 0)
        {
            for (int size = 500; size >= 2; size--)
            {
                int triangle_number = size * (size - 1) / 2;

                if (triangle_number <= remaining)
                {
                    group_sizes.push_back(size);
                    remaining -= triangle_number;
                    break;
                }
            }
        }

        int total_points = 0;
        for (int group : group_sizes)
            total_points += group;

        std::cout << total_points << "\n";

        for (size_t index = 0; index < group_sizes.size(); index++)
        {
            int group = group_sizes[index];
            int base_x = index * 1000;
            int y_coordinate = index;

            for (int j = 0; j < group; j++)
            {
                std::cout << (base_x + j) << " " << y_coordinate << "\n";
            }
        }
    }

    return 0;
}

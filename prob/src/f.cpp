#include <bits/stdc++.h>

#define FAST_IO                       \
    std::ios::sync_with_stdio(false); \
    std::cin.tie(nullptr);

#define ll long long

void processTestCase()
{
    int size;
    unsigned int value;
    std::cin >> size >> value;

    int maxIndex = size - 1;

    for (int index = 1; index <= size; index++)
    {
        int current = index - 1;

        if ((current | maxIndex) == maxIndex)
            std::cout << value << (index == size ? "\n" : " ");
        else
            std::cout << 0 << (index == size ? "\n" : " ");
    }
}

int main()
{
    FAST_IO

    int testCases;
    std::cin >> testCases;

    while (testCases--)
    {
        processTestCase();
    }

    return 0;
}

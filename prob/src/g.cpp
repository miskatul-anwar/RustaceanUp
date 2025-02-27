#include <iostream>
#include <vector>

const long long MOD = 1000000007;
const long long INV2 = 500000004;
const long long INV6 = 166666668;

inline long long modMul(long long a, long long b)
{
    __int128 t = a;
    t *= b;
    t %= MOD;
    return static_cast<long long>(t);
}

long long modSum(long long L, long long R)
{
    long long count = (R - L + 1) % MOD;
    long long sum = modMul((L + R) % MOD, count);
    return modMul(sum, INV2);
}

long long modSumSq(long long L, long long R)
{
    auto f = [](long long x) -> long long
    {
        long long term = modMul(modMul(x % MOD, (x + 1) % MOD), ((2 * x + 1) % MOD));
        return modMul(term, INV6);
    };

    long long res = f(R) - f(L - 1);
    return (res + MOD) % MOD;
}

long long intRoot(long long n, int r)
{
    long long lo = 1, hi = n, ans = 1;

    while (lo <= hi)
    {
        long long mid = (lo + hi) / 2;
        __int128 prod = 1;
        bool ok = true;

        for (int i = 0; i < r; i++)
        {
            prod *= mid;
            if (prod > n)
            {
                ok = false;
                break;
            }
        }

        if (ok)
        {
            ans = mid;
            lo = mid + 1;
        }
        else
        {
            hi = mid - 1;
        }
    }
    return ans;
}

long long revInBase(long long n, long long p)
{
    long long res = 0;
    long long base = p % MOD;

    while (n > 0)
    {
        int d = n % p;
        n /= p;
        res = (modMul(res, base) + d) % MOD;
    }

    return res;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int test_cases;
    std::cin >> test_cases;

    while (test_cases--)
    {
        long long n, k;
        std::cin >> n >> k;

        long long result = 0;

        if (n == 1)
        {
            std::cout << ((k - 1) % MOD + MOD) % MOD << "\n";
            continue;
        }

        if (k > n)
        {
            result = modMul(n % MOD, (k - n) % MOD);
        }

        int Lmax = 0;
        for (long long temp = n; temp > 0; temp /= 2)
        {
            Lmax++;
        }

        for (int L = 2; L <= Lmax; L++)
        {
            long long A = std::max(intRoot(n, L) + 1, 2LL);
            long long B = std::min(intRoot(n, L - 1), k);

            if (A > B)
                continue;

            if (L == 2)
            {
                long long groupSum = 0;
                long long p = A;

                while (p <= B)
                {
                    long long f = n / p;
                    long long r = std::min(B, n / f);

                    long long sumP = modSum(p, r);
                    long long part1 = modMul(n % MOD, sumP);

                    long long sumP2 = modSumSq(p, r);
                    long long count = ((r - p + 1) % MOD);
                    long long part2 = modMul(f % MOD, ((sumP2 - count) % MOD + MOD) % MOD);

                    long long groupVal = (part1 - part2 + MOD) % MOD;
                    groupSum = (groupSum + groupVal) % MOD;

                    p = r + 1;
                }

                result = (result + groupSum) % MOD;
            }
            else
            {
                for (long long p = A; p <= B; p++)
                {
                    result = (result + revInBase(n, p)) % MOD;
                }
            }
        }

        std::cout << result % MOD << "\n";
    }

    return 0;
}

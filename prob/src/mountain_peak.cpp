#include <vector>

#include <stack>

#include <iostream>

using namespace std;

vector<int> next_higher_peak(vector<int> &heights)
{
  int max = -1;
  int curr_max = 0;

  for (int i = heights.size() - 1; i >= 0; --i)
  {
    curr_max = heights[i];

    if (heights[i] >= max)
    {
      heights[i] = -1;
    }
    else
    {
      heights[i] = max;
    }

    if (max < curr_max)
    {
      max = curr_max;
    }
  }

  return heights;
}

int main()
{
  int n;

  cin >> n; // Read the number of peaks from the user

  vector<int> heights(n);

  for (int i = 0; i < n; ++i)
  {
    cin >> heights[i]; // Read each peak height from the user
  }

  vector<int> result = next_higher_peak(heights); // Call the function to get the result

  // Print result

  for (int height : result)
  {
    cout << height << " "; // Output each element in the result vector
  }
  cout << endl;

  return 0;
}
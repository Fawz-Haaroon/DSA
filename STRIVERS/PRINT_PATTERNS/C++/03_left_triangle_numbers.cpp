#include <iostream>

using namespace std;

void pattern_x(int n)
{
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j <= i; j++)
        {
            cout << j + 1;
        }

        cout << '\n';
    }
}

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;

    cin >> n;

    if (n <= 0)
    {
        cerr << "Error: n must be greater than 0.\n";
        return EXIT_FAILURE;
    }

    pattern_x(n);

    return EXIT_SUCCESS;
}

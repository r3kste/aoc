#include <bits/stdc++.h>
using namespace std;

#define MOD (LL)(1e9 + 7)
#define fastio                        \
    ios_base::sync_with_stdio(false); \
    cin.tie(NULL);

#define endl "\n"
#define yesno(a) cout << ((a) ? "Yes" : "No");

#define F first
#define S second
#define mp make_pair
#define pb push_back
#define all(a) (a).begin(), (a).end()
#define rall(a) (a).rbegin(), (a).rend()

typedef long long int ll;
typedef unsigned long long int LL;
typedef pair<int, int> ii;

typedef vector<int> vi;
typedef vector<pair<int, int>> vii;
typedef vector<long long int> vll;

#define n 100
void rrr (char arr[n][n]) {
    for (int i = 0; i < n / 2; i++) {
        for (int j = i; j < n - i - 1; j++) {
            // Swapping elements after each iteration in Anticlockwise direction
            int temp = arr[i][j];
            arr[i][j] = arr[j][n - i - 1];
            arr[j][n - i - 1] = arr[n - i - 1][n - j - 1];
            arr[n - i - 1][n - j - 1] = arr[n - j - 1][i];
            arr[n - j - 1][i] = temp;
        }
    }
}

void print (char a[n][n]) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            if (a[j][i] != '#' && a[j][i] != 'O' && a[j][i] != '.') {
                // cout << ".";
                a[j][i] = '.';
            }

            cout << a[j][i] << " ";
        }

        cout << "\n";
    }

    cout << "\n\n";
}

int solve() {
    // fastio;
    ifstream input;
    input.open ("input.txt");
    string line;
    // n *= 10;
    // vector<vector<char>> one (n, vector<char> (n, '.'));
    char one[n][n];
    ll ans = 0;
    vll anss;

    if (input.is_open()) {
        char ch;
        int i = 0;
        int j = 0;

        while (getline (input, line)) {
            for (size_t h = 0; h < line.size(); h++) {
                ch = line[h];

                if (i == n) {
                    break;
                }

                if (j == n) {
                    i++;
                    j = 0;
                }

                if (ch == '\n') {
                    continue;
                }

                one[j][i] = ch;
                j++;
            }
        }

        input.close();
    }

    // for (int f = 1; f <= 100; f++)
    // {
    char two[n][n];

    for (int g = 1; g <= 4 * 1000; g++) {
        ans = 0;

        //Move one UP and store in two
        for (int i = 0; i < n; i++) { //column
            int c = 0;
            int h = -1;

            for (int j = 0; j < n; j++) { //inside column elements
                char ch = one[i][j];

                if (ch == 'O') {
                    c++;
                }

                if (ch == '#') {
                    for (int k = 0; k < c; k++) {
                        two[i][h + 1 + k] = 'O';
                    }

                    for (int k = c; k < j; k++) {
                        two[i][h + 1 + k] = '.';
                    }

                    two[i][j] = '#';
                    c = 0;
                    h = j;
                }
            }

            if (c != 0) {
                for (int k = 0; k < c; k++) {
                    two[i][h + 1 + k] = 'O';
                }

                for (int k = c; k < n; k++) {
                    two[i][h + 1 + k] = '.';
                }
            }
        }

        if ((g + 1) % 4 == 0) {
            anss.pb (ans);
        }

        rrr (two);

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                one[i][j] = two[i][j];
            }
        }
    }

    // print (one);

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            char ch = one[j][i];

            if (ch == 'O') {
                ans += (n - i);
            }
        }
    }

    cout << ans << "\n";
    return 0;
}

int main() {
    // fastio;
    int t = 1;

    while (t--) {
        solve();
        cout << "\n";
    }
}
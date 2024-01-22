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
#define pb emplace_back
#define all(a) (a).begin(), (a).end()
#define rall(a) (a).rbegin(), (a).rend()

typedef long long int ll;
typedef unsigned long long int LL;
typedef pair<int, int> ii;

typedef vector<int> vi;
typedef vector<pair<int, int>> vii;
typedef vector<long long int> vll;

int solve() {
    fastio
    ifstream input;
    std::filesystem::path path (__FILE__);
    path = path.parent_path();
    path /= "input.txt";
    input.open (path);
    string line;
    int n = 10;
    n *= 10;
    vector<vector<char>> one (n, vector<char> (n, '.'));
    vector<vector<char>> two (n, vector<char> (n, '.'));

    if (input.is_open()) {
        char ch;
        int i = 0;
        int j = 0;

        while (getline (input, line)) {
            for (char h : line) {
                ch = h;

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

                two[i][j] = '#';
                c = 0;
                h = j;
            }
        }

        if (c != 0) {
            for (int k = 0; k < c; k++) {
                two[i][h + 1 + k] = 'O';
            }
        }
    }

    ll ans = 0;

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            char ch = two[j][i];

            if (ch == 'O') {
                ans += (n - i);
            }
        }
    }

    cout << ans;
    return 0;
}

int main() {
    fastio
    int t = 1;

    while (t--) {
        solve();
        cout << "\n";
    }
}
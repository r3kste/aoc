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

int n = 110;
vector<vector<char>> inp(n, vector<char>(n, '.'));
vector<vector<bool>> ene(n, vector<bool>(n, false));
map<pair<char, pair<int, int>>, bool> p;

void reset() {
    ene = vector<vector<bool>>(n, vector<bool>(n, false));
    p.clear();
}

void traverse(char facing, int i, int j) {
    if (!(i < 0 || i > n - 1 || j < 0 || j > n - 1)) {
        p[mp(facing, mp(i, j))] = true;
        char ch = inp[i][j];
        ene[i][j] = true;

        switch (facing) {
            case 'r':
                if (ch == '.' || ch == '-') {
                    if (!p[mp('r', mp(i, j + 1))]) {
                        traverse('r', i, j + 1);
                    }
                } else if (ch == '\\') {
                    if (!p[mp('d', mp(i + 1, j))]) {
                        traverse('d', i + 1, j);
                    }
                } else if (ch == '/') {
                    if (!p[mp('u', mp(i - 1, j))]) {
                        traverse('u', i - 1, j);
                    }
                } else if (ch == '|') {
                    if (!p[mp('u', mp(i - 1, j))]) {
                        traverse('u', i - 1, j);
                    }

                    if (!p[mp('d', mp(i + 1, j))]) {
                        traverse('d', i + 1, j);
                    }
                }

                break;

            case 'l':
                if (ch == '.' || ch == '-') {
                    if (!p[mp('l', mp(i, j - 1))]) {
                        traverse('l', i, j - 1);
                    }
                } else if (ch == '/') {
                    if (!p[mp('d', mp(i + 1, j))]) {
                        traverse('d', i + 1, j);
                    }
                } else if (ch == '\\') {
                    if (!p[mp('u', mp(i - 1, j))]) {
                        traverse('u', i - 1, j);
                    }
                } else if (ch == '|') {
                    if (!p[mp('u', mp(i - 1, j))]) {
                        traverse('u', i - 1, j);
                    }

                    if (!p[mp('d', mp(i + 1, j))]) {
                        traverse('d', i + 1, j);
                    }
                }

                break;

            case 'u':
                if (ch == '.' || ch == '|') {
                    if (!p[mp('u', mp(i - 1, j))]) {
                        traverse('u', i - 1, j);
                    }
                } else if (ch == '\\') {
                    if (!p[mp('l', mp(i, j - 1))]) {
                        traverse('l', i, j - 1);
                    }
                } else if (ch == '/') {
                    if (!p[mp('r', mp(i, j + 1))]) {
                        traverse('r', i, j + 1);
                    }
                } else if (ch == '-') {
                    if (!p[mp('r', mp(i, j + 1))]) {
                        traverse('r', i, j + 1);
                    }

                    if (!p[mp('l', mp(i, j - 1))]) {
                        traverse('l', i, j - 1);
                    }
                }

                break;

            case 'd':
                if (ch == '.' || ch == '|') {
                    if (!p[mp('d', mp(i + 1, j))]) {
                        traverse('d', i + 1, j);
                    }
                } else if (ch == '\\') {
                    if (!p[mp('r', mp(i, j + 1))]) {
                        traverse('r', i, j + 1);
                    }
                } else if (ch == '/') {
                    if (!p[mp('l', mp(i, j - 1))]) {
                        traverse('l', i, j - 1);
                    }
                } else if (ch == '-') {
                    if (!p[mp('r', mp(i, j + 1))]) {
                        traverse('r', i, j + 1);
                    }

                    if (!p[mp('l', mp(i, j - 1))]) {
                        traverse('l', i, j - 1);
                    }
                }

                break;

            default:
                break;
        }
    }
}

int solve() {
    fastio
    ifstream input;
    std::filesystem::path path(__FILE__);
    path = path.parent_path();
    path /= "input.txt";
    input.open(path);
    string line;

    if (input.is_open()) {
        char ch;
        int i = 0;
        int j = 0;

        while (getline(input, line)) {
            for (char h: line) {
                ch = h;

                if (j == n) {
                    i++;
                    j = 0;
                }

                if (i == n) {
                    break;
                }

                if (ch == '\n') {
                    continue;
                }

                inp[i][j] = ch;
                j++;
            }
        }

        input.close();
    }

    ll ans = 0;
    ll c;

    for (int h = 0; h < n; h++) {
        traverse('r', h, 0);
        c = 0;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (ene[i][j]) {
                    c++;
                }
            }
        }

        if (c > ans) {
            ans = c;
        }

        reset();
        traverse('l', h, n - 1);
        c = 0;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (ene[i][j]) {
                    c++;
                }
            }
        }

        if (c > ans) {
            ans = c;
        }

        reset();
        traverse('d', 0, h);
        c = 0;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (ene[i][j]) {
                    c++;
                }
            }
        }

        if (c > ans) {
            ans = c;
        }

        reset();
        traverse('u', n - 1, h);
        c = 0;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (ene[i][j]) {
                    c++;
                }
            }
        }

        if (c > ans) {
            ans = c;
        }

        reset();
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
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
    // int c = 5;
    // int r = 5;
    int c = 140;
    int r = 140;
    vector<vector<char>> inp (r, vector<char> (c, '.'));
    vector<vector<int>> dist (r, vector<int> (c, 0));
    int starti, startj;

    if (input.is_open()) {
        char ch;
        int i = 0;
        int j = 0;

        while (getline (input, line)) {
            for (char h : line) {
                ch = h;

                if (i == r) {
                    break;
                }

                if (j == c) {
                    i++;
                    j = 0;
                }

                if (ch == 'S') {
                    starti = i;
                    startj = j;
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

    int i, j, k;
    //Path right
    i = starti;
    j = startj + 1;
    k = 1;
    char from = 'l';

    while (! (i == starti && j == startj)) {
        char pos = inp[i][j];
        dist[i][j] = k++;

        if (from == 'l') {
            if (pos == '-') {
                j++;
                from = 'l';
            } else if (pos == 'J') {
                i--;
                from = 'd';
            } else {
                i++;
                from = 'u';
            }
        } else if (from == 'u') {
            if (pos == '|') {
                i++;
                from = 'u';
            } else if (pos == 'J') {
                j--;
                from = 'r';
            } else {
                j++;
                from = 'l';
            }
        } else if (from == 'd') {
            if (pos == '|') {
                i--;
                from = 'd';
            } else if (pos == 'F') {
                j++;
                from = 'l';
            } else {
                j--;
                from = 'r';
            }
        } else {
            if (pos == '-') {
                j--;
                from = 'r';
            } else if (pos == 'F') {
                i++;
                from = 'u';
            } else {
                i--;
                from = 'd';
            }
        }
    }

    //Path down
    i = starti + 1;
    j = startj;
    k = 1;
    from = 'u';

    while (! (i == starti && j == startj)) {
        char pos = inp[i][j];
        dist[i][j] = min (k++, dist[i][j]);

        if (from == 'l') {
            if (pos == '-') {
                j++;
                from = 'l';
            } else if (pos == 'J') {
                i--;
                from = 'd';
            } else {
                i++;
                from = 'u';
            }
        } else if (from == 'u') {
            if (pos == '|') {
                i++;
                from = 'u';
            } else if (pos == 'J') {
                j--;
                from = 'r';
            } else {
                j++;
                from = 'l';
            }
        } else if (from == 'd') {
            if (pos == '|') {
                i--;
                from = 'd';
            } else if (pos == 'F') {
                j++;
                from = 'l';
            } else {
                j--;
                from = 'r';
            }
        } else {
            if (pos == '-') {
                j--;
                from = 'r';
            } else if (pos == 'F') {
                i++;
                from = 'u';
            } else {
                i--;
                from = 'd';
            }
        }
    }

    ll max = -MOD;

    for (int i = 0; i < r; i++) {
        for (int j = 0; j < c; j++) {
            if (dist[i][j] > max) {
                max = dist[i][j];
            }
        }
    }

    cout << max;
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
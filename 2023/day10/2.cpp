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

int solve() {
    fastio;
    ifstream input;
    input.open ("input.txt");
    string line;
    int c = 140;
    int r = 140;
    // int c = 5;
    // int r = 5;
    // int c = 11;
    // int r = 9;
    // int c = 20;
    // int r = 10;
    vector<vector<char>> inp (r, vector<char> (c, '.'));
    vector<vector<bool>> done (r, vector<bool> (c, false));
    int starti, startj;
    char prepos;

    if (input.is_open()) {
        char ch;
        int i = 0;
        int j = 0;

        while (getline (input, line)) {
            for (size_t h = 0; h < line.size(); h++) {
                ch = line[h];

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

    int i, j;
    done[starti][startj] = true;
    //Path right
    i = starti;
    j = startj + 1;
    char from = 'l';

    while (! (i == starti && j == startj)) {
        char pos = inp[i][j];
        done[i][j] = true;

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
        } else if (from == 'r') {
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

    ll ans = 0;

    for (int i = 0; i < r; i++) {
        int count = 0;

        for (int j = 0; j < c; j++) {
            if (done[i][j]) {
                if (inp[i][j] == '-') {
                    continue;
                }

                count++;

                if (inp[i][j] == 'J' && prepos == 'F') {
                    count--;
                }

                if (inp[i][j] == '7' && prepos == 'L') {
                    count --;
                }

                prepos = inp[i][j];
            } else {
                if (count % 2) {
                    ans++;
                }
            }
        }
    }

    cout << ans;
    return 0;
}

int main() {
    fastio;
    int t = 1;

    while (t--) {
        solve();
        cout << "\n";
    }
}
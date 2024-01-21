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
    vector<vector<char>> inp (r, vector<char> (c, '.'));
    ll sum = 0;

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

                if (ch == '\n') {
                    continue;
                }

                inp[i][j] = ch;
                j++;
            }
        }

        input.close();
    }

    map<int, vector<int>> mp;
    int k;
    string temp = "";

    for (int i = 0; i < r; i++) {
        for (int j = 0; j < c; j++) {
            if (inp[i][j] != '.') {
                char ch = inp[i][j];
                temp = "";
                temp += ch;

                if (ch >= 48 && ch <= 57) {
                    for (k = j + 1; k < c; k++) {
                        if (! (inp[i][k] >= 48 && inp[i][k] <= 57)) {
                            k--;
                            break;
                        } else {
                            temp += inp[i][k];
                        }
                    }

                    int left = j - 1;
                    int right = k + 1;
                    int top = i - 1;
                    int bot = i + 1;

                    for (int l = max (0, top); l <= min (bot, r - 1); l++) {
                        for (int m = max (0, left); m <= min (right, c - 1); m++) {
                            if (l == top || l == bot || m == left || m == right) {
                                if (inp[l][m] != '.' && ! (inp[l][m] >= 48 && inp[l][m] <= 57)) {
                                    sum += stoi (temp);
                                    goto labellesgo;
                                }
                            }
                        }
                    }

labellesgo:
                    j = k + 1;
                }
            }
        }
    }

    cout << sum;
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

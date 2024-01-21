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
    int r = 140;
    int c = 140;
    vector<vector<char>> inp (r, vector<char> (c, '.'));
    vector<vector<char>> index (r, vector<char> (c, '.'));
    vii ind;
    vi rval (r, 1000000);
    vi cval (c, 1000000);

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

                if (ch == '#') {
                    ind.pb (mp (i, j));
                    rval[i] = 1;
                    cval[j] = 1;
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

    vi psr;
    psr.pb (0);

    for (size_t i = 0; i < rval.size(); i++) {
        psr.pb (psr[i] + rval[i]);
    }

    vi psc;
    psc.pb (0);

    for (size_t i = 0; i < cval.size(); i++) {
        psc.pb (psc[i] + cval[i]);
    }

    int fi, fj, si, sj;
    ll ans = 0;

    for (size_t i = 0; i < ind.size(); i++) {
        fi = ind[i].F;
        fj = ind[i].S;

        for (size_t j = i + 1; j < ind.size(); j++) {
            si = ind[j].F;
            sj = ind[j].S;
            ans += (abs (psr[si] - psr[fi]) + abs (psc[sj] - psc[fj]));
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
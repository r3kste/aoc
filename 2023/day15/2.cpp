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
    ll val;
    ifstream input;
    std::filesystem::path path (__FILE__);
    path = path.parent_path();
    path /= "input.txt";
    input.open (path);
    string line;
    ll ans = 0;
    vector<vector<pair<string, int>>> p (256, vector<pair<string, int>> (0, mp ("", 0)));

    if (input.is_open()) {
        getline (input, line);
        stringstream words (line);
        string subword;

        while (getline (words, subword, ',')) {
            string label;
            int op = 0;
            int foc = 0;

            for (size_t j = 0; j < subword.size(); j++) {
                char i = subword[j];

                if (i == '-') {
                    op = -1;
                    break;
                } else if (i == '=') {
                    op = 1;
                    foc = (subword[j + 1]) - int ('0');
                    break;
                } else {
                    label += (i);
                }
            }

            val = 0;

            for (auto i : label) {
                val += int (i);
                val *= 17;
                val %= 256;
            }

            if (op == 1) {
                bool flag = true;
                vector<pair<string, int>> t;

                // for (auto i : p[val])
                for (auto &j : p[val]) {
                    auto i = j;

                    if (i.F == label) {
                        j.S = foc;
                        flag = false;
                        break;
                    }
                }

                if (flag) {
                    p[val].pb (mp (label, foc));
                }
            } else if (op == -1) {
                vector<pair<string, int>> t;

                for (const auto &i : p[val]) {
                    if (i.F != label) {
                        t.pb (mp (i.F, i.S));
                    }
                }

                p[val] = t;
            }
        }

        input.close();
    }

    for (int i = 0; i < 256; i++) {
        for (size_t j = 0; j < p[i].size(); j++) {
            ans += (i + 1) * (j + 1) * (p[i][j].S);
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
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

bool check (vector<vector<char>> &A, int i, int j) {
    if (A[i] != A[j]) {
        return false;
    }

    if (i == 0 || j == (int) A.size() - 1) {
        return (A[i] == A[j]);
    } else {
        return check (A, i - 1, j + 1);
    }
}

int solve() {
    fastio
    ifstream input;
    std::filesystem::path path (__FILE__);
    path = path.parent_path();
    path /= "input.txt";
    input.open (path);
    string line;
    vector<vector<char>> I;
    vector<vector<char>> J;
    size_t sum = 0;

    if (input.is_open()) {
        while (getline (input, line)) {
            vector<char> x;

            for (auto i : line) {
                x.pb (i);
            }

            if (!x.empty()) {
                I.pb (x);
            }

            if (line.empty()) {
                for (size_t j = 0; j < I[0].size(); j++) {
                    for (auto &i : I) {
                        x.pb (i[j]);
                    }

                    J.pb (x);
                    x.clear();
                }

                for (size_t i = 0; i < I.size(); i++) {
                    if (I[i] == I[i + 1]) {
                        if (check (I, i, i + 1)) {
                            sum += (100 * (i + 1));
                        }
                    }
                }

                for (size_t i = 0; i < J.size(); i++) {
                    if (J[i] == J[i + 1]) {
                        if (check (J, i, i + 1)) {
                            sum += (i + 1);
                        }
                    }
                }

                I.clear();
                J.clear();
            }
        }

        input.close();
    }

    cout << sum;
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
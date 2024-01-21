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
int correction (vector<char> one, vector<char> two) {
    int c = 0;

    for (int i = 0; i < (int) one.size(); i++) {
        if (one[i] != two[i]) {
            c++;
        }
    }

    return c;
}
int check (vector<vector<char>> A, int i, int j) {
    if (i == 0 || j == (int) A.size() - 1) {
        return correction (A[i], A[j]);
    }

    if (A[i] != A[j]) {
        return correction (A[i], A[j]) + check (A, i - 1, j + 1);
    } else {
        return check (A, i - 1, j + 1);
    }

    return 0;
}
int solve() {
    fastio;
    ifstream input;
    input.open ("input.txt");
    string line;
    vector<vector<char>> I;
    vector<vector<char>> J;
    ll sum = 0;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            vector<char> x;

            for (auto i : line) {
                x.pb (i);
            }

            if (x.size() != 0) {
                I.pb (x);
            }

            if (line == "") {
                for (size_t j = 0; j < I[0].size(); j++) {
                    vector<char> x;

                    for (size_t i = 0; i < I.size(); i++) {
                        x.pb (I[i][j]);
                    }

                    J.pb (x);
                    x.clear();
                }

                for (size_t i = 0; i < I.size() - 1; i++) {
                    if (check (I, i, i + 1) == 1) {
                        sum += (100 * (i + 1));
                    }
                }

                for (size_t i = 0; i < J.size() - 1; i++) {
                    if (check (J, i, i + 1) == 1) {
                        sum += (i + 1);
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
    fastio;
    int t = 1;

    while (t--) {
        solve();
        cout << "\n";
    }
}
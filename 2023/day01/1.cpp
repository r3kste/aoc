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
    ll sum = 0;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            int a = 0;
            int b = 0;
            bool flag = true;

            for (size_t i = 0; i < line.size(); i++) {
                if (int (line[i]) >= 48 && int (line[i]) <= 57) {
                    if (flag) {
                        a = int (line[i]) - 48;
                        b = a;
                        flag = false;
                    } else {
                        b = int (line[i]) - 48;
                    }
                }
            }

            sum += (10 * a) + b;
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

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
    ll val = 0;
    ifstream input;
    input.open ("input.txt");
    string line;
    ll ans = 0;

    if (input.is_open()) {
        getline (input, line);
        stringstream words (line);
        string subword;

        while (getline (words, subword, ',')) {
            val = 0;

            for (auto i : subword) {
                val += int (i);
                val *= 17;
                val %= 256;
            }

            ans += val;
        }

        input.close();
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
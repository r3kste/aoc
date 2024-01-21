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
    std::filesystem::path path(__FILE__);
    path = path.parent_path();
    path /= "input.txt";
    input.open(path);
    string line;
    vi a;
    vi b;
    vi last;
    ll ans = 0;

    if (input.is_open()) {
        label:

        while (getline(input, line)) {
            stringstream words(line);
            string word;
            a.clear();
            b.clear();
            last.clear();

            while (getline(words, word, ' ')) {
                a.pb(stoi(word));
            }

            bool flag = true;
            last.clear();

            while (true) {
                if (flag) {
                    for (size_t i = 0; i < a.size() - 1; i++) {
                        b.pb(a[i + 1] - a[i]);
                    }

                    last.pb(a[a.size() - 1]);
                    a.clear();
                    flag = false;

                    if (b == vi(b.size(), 0)) {
                        ll sum = 0;

                        for (int j: last) {
                            sum += j;
                        }

                        ans += sum;
                        goto label;
                    }
                } else {
                    for (size_t i = 0; i < b.size() - 1; i++) {
                        a.pb(b[i + 1] - b[i]);
                    }

                    last.pb(b[b.size() - 1]);
                    b.clear();
                    flag = true;

                    if (a == vi(a.size(), 0)) {
                        ll sum = 0;

                        for (int j: last) {
                            sum += j;
                        }

                        ans += sum;
                        goto label;
                    }
                }
            }
        }

        input.close();
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
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

int f (vector<char> p, vi n) {
    vector<char> p1 = p;
    vector<char> p2 = p;
    vi seq;

    for (size_t i = 0; i < p.size(); i++) {
        if (p[i] == '?') {
            p1[i] = '.';
            p2[i] = '#';
            return f (p1, n) + f (p2, n);
        }
    }

    int c = 1;

    for (size_t i = 0; i < p.size() - 1; i++) {
        if (p[i] == '#' && p[i + 1] == '#') {
            c++;
        }

        if (p[i] == '#' && p[i + 1] != '#') {
            seq.pb (c);
            c = 1;
        }
    }

    if (p[p.size() - 1] == '#') {
        seq.pb (c);
    }

    int aa = ((seq == n) ? 1 : 0);
    return aa;
}

int solve() {
    fastio;
    ifstream input;
    input.open ("input.txt");
    string line;
    ll ans = 0;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            vector<char> p;
            vi n;
            stringstream words (line);
            string word;
            getline (words, word, ' ');

            for (auto i : word) {
                p.pb (i);
            }

            getline (words, word, ' ');
            stringstream nums (word);
            string num;

            while (getline (nums, num, ',')) {
                n.pb (stoi (num));
            }

            //f(line)=no of ways
            //f(line)=f(line with first ? to.) + f(line with first ? to #)
            ans += f (p, n);
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
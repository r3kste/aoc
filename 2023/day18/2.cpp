#include <bits/stdc++.h>
using namespace std;

#define MOD (LL)(1e9 + 7)
#define fastio                        \
    ios_base::sync_with_stdio(false); \
    cin.tie(NULL);

#define endl "\n"
#define yesno(a) cout << ((a) ? "Yes" : "No");

#define x first
#define F first
#define y second
#define S second
#define mp make_pair
#define pb push_back
#define all(a) (a).begin(), (a).end()
#define rall(a) (a).rbegin(), (a).rend()

typedef long long int ll;
typedef pair<int, int> ii;

typedef vector<int> vi;
typedef vector<pair<ll, ll>> vii;
typedef vector<long long int> vll;
ll polygon_area (vii fig) {
    ll res = 0;

    for (size_t i = 0; i < fig.size(); i++) {
        ii p = i ? fig[i - 1] : fig.back();
        ii q = fig[i];
        ll a = p.x;
        ll b = p.y;
        ll c = q.x;
        ll d = q.y;
        res +=  (a * d - b * c);
    }

    return abs (res);
}
int solve() {
    fastio;
    ifstream input;
    input.open ("input.txt");
    string line;
    vector<pair<char, int>> p;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            stringstream words (line);
            string word;
            getline (words, word, ' ');
            getline (words, word, ' ');
            getline (words, word, ' ');
            char dir;

            switch (word[word.size() - 2]) {
            case '0':
                dir = 'R';
                break;

            case '1':
                dir = 'D';
                break;

            case '2':
                dir = 'L';
                break;

            case '3':
                dir = 'U';
                break;
            }

            word = word.substr (2, 5);
            istringstream h (word);
            ll n;
            h >> hex >> n;
            p.pb (mp (dir, n));
        }

        input.close();
    }

    ll X = 0;
    ll Y = 0;
    vii corner;
    corner.pb (mp (0, 0));
    ll b = 0;

    for (auto i : p) {
        b += i.S;

        switch (i.F) {
        case 'R':
            X += i.S;
            corner.pb (mp (X, Y));
            break;

        case 'L':
            X -= i.S;
            corner.pb (mp (X, Y));
            break;

        case 'U':
            Y += i.S;
            corner.pb (mp (X, Y));
            break;

        case 'D':
            Y -= i.S;
            corner.pb (mp (X, Y));
            break;

        default:
            break;
        }
    }

    corner.pb (mp (0, 0));
    ll A = polygon_area (corner);
    cout <<  (A + b + 2) / 2;
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
#include <bits/stdc++.h>
#include <numeric>

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

ll lllcccmmm (ll a, ll b) {
    return (a * b) / __gcd (a, b);
}

int solve() {
    fastio;
    ifstream input;
    input.open ("input.txt");
    string line;
    getline (input, line);
    vector<char> instructions;

    for (size_t i = 0; i < line.size(); i++) {
        instructions.pb (line[i]);
    }

    getline (input, line);
    map<string, pair<string, string>> p;
    vector<string> position;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            stringstream words (line);
            string word;
            getline (words, word, ' ');
            string FF;
            FF = word;
            reverse (all (FF));

            if (FF[0] == 'A') {
                position.pb (FF);
            }

            getline (words, word, ' ');
            getline (words, word, ' ');
            word = word.substr (1, 3);
            reverse (all (word));
            string SF = word;
            getline (words, word, ' ');
            word = word.substr (0, 3);
            reverse (all (word));
            string SS = word;
            p[FF] = mp (SF, SS);
        }

        input.close();
    }

    ll count = 0;
    vll cycles (position.size());

    for (size_t h = 0; h < position.size(); h++) {
        count = 0;

        for (size_t i = 0; i < instructions.size(); i = (i + 1) % instructions.size()) {
            count ++;
            string pos = position[h];
            char what = instructions[i];
            position[h] = (what == 'R') ? p[position[h]].S : p[position[h]].F;

            if (position[h][0] == 'Z') {
                cycles[h] = count;
                break;
            }
        }
    }

    ll lcm_all = accumulate (cycles.begin(), cycles.end(), 1LL, lllcccmmm);
    cout << lcm_all;
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
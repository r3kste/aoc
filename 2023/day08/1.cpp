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
    getline(input, line);
    vector<char> instructions;

    for (char i: line) {
        instructions.pb(i);
    }

    getline(input, line);
    map<string, pair<string, string>> p;

    if (input.is_open()) {
        while (getline(input, line)) {
            stringstream words(line);
            string word;
            getline(words, word, ' ');
            string FF;

            if (isalpha(word[0])) {
                FF = word;
            }

            getline(words, word, ' ');
            getline(words, word, ' ');
            word = word.substr(1, 3);
            string SF = word;
            getline(words, word, ' ');
            word = word.substr(0, 3);
            string SS = word;
            p[FF] = mp(SF, SS);
        }

        input.close();
    }

    string position = "AAA";
    int c = 0;

    for (int i = 0; true; i = (i + 1) % instructions.size()) {
        char what = instructions[i];
        position = (what == 'R') ? p[position].S : p[position].F;
        c++;

        if (position == "ZZZ") {
            cout << c;
            break;
        }
    }

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
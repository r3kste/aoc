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
    string word;
    bool flag = false;
    bool flag2 = false;
    vi a;
    vi b;
    int c = 0;
    ll sum = 0;

    if (input.is_open()) {
        while ( getline (input, word, ' ') ) {
            while (word == "") {
                getline (input, word, ' ');
            }

            if (word == "Card") {
                getline (input, word, ' ');
            }

            while (word == "") {
                getline (input, word, ' ');
            }

            if (word[word.size() - 1] == ':') {
                getline (input, word, ' ');
                flag = false;
            }

            while (word == "") {
                getline (input, word, ' ');
            }

            if (word[word.size() - 1] == '|') {
                getline (input, word, ' ');
                flag = true;
            }

            while (word == "") {
                getline (input, word, ' ');
            }

            if (word.find ("\n") != string::npos) {
                // word=word.substr(word.find("\n"),word.size()-word.find("\n"));
                word = word.substr (0, word.find ("\n"));
                flag2 = true;
            }

            if (flag) {
                b.pb (stoi (word));
            } else {
                a.pb (stoi (word));
            }

            if (flag2) {
                for (size_t i = 0; i < a.size(); i++) {
                    for (size_t j = 0; j < b.size(); j++) {
                        if (a[i] == b[j]) {
                            c++;
                        }
                    }
                }

                sum += ll (pow (2, c - 1));
                a.clear();
                b.clear();
                flag2 = false;
                c = 0;
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

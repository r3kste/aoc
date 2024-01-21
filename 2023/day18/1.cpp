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

#define order 500
char ground[order][order];
void floodfill (int i, int j) {
    if  (i >= 0 && i < order && j >= 0 && j < order) {
        ground[i][j] = '#';

        if (ground[i][j + 1] != '#') {
            floodfill (i, j + 1);
        }

        if (ground[i][j - 1] != '#') {
            floodfill (i, j - 1);
        }

        if (ground[i + 1][j] != '#') {
            floodfill (i + 1, j);
        }

        if (ground[i - 1][j] != '#') {
            floodfill (i - 1, j);
        }
    }
}
int solve() {
    fastio;
    ifstream input;
    ofstream output;
    input.open ("input.txt");
    output.open ("output.txt");
    string line;
    vector<pair<pair<char, int>, string>> p;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            stringstream words (line);
            string word;
            string word2;
            string word3;
            getline (words, word, ' ');
            getline (words, word2, ' ');
            getline (words, word3, ' ');
            p.pb (mp (mp (word[0], stoi (word2)), word3));
        }

        input.close();
    }

    for (int i = 0; i < order; i++) {
        for (int j = 0; j < order; j++) {
            ground[i][j] = '.';
        }
    }

    int pi = order / 2;
    int pj = order / 2;
    ground[pi][pj] = '#';

    for (auto a : p) {
        int n = a.F.S;
        char dir = a.F.F;

        for (int i = 1; i <= n; i++) {
            switch (dir) {
            case 'L':
                ground[pi][--pj] = '#';
                break;

            case 'R':
                ground[pi][++pj] = '#';
                break;

            case 'U':
                ground[--pi][pj] = '#';
                break;

            case 'D':
                ground[++pi][pj] = '#';
                break;

            default:
                break;
            }
        }
    }

    floodfill (90, 50);
    ll c = 0;

    for (int i = 0; i < order; i++) {
        for (int j = 0; j < order; j++) {
            output << ground[i][j];

            if (ground[i][j] == '#') {
                c++;
            }
        }

        output << "\n";
    }

    cout << c;
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
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
    vii data (4);
    int k = 0;
    int l = 0;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            stringstream words (line);
            string temp;
            getline (words, temp, ' ');

            if (temp == "Time:") {
                while (getline (words, temp, ' ')) {
                    while (temp == "") {
                        getline (words, temp, ' ');
                    }

                    if (isdigit (temp[0])) {
                        data[k++].F = stoi (temp);
                    }
                }
            } else if (temp == "Distance:") {
                while (getline (words, temp, ' ')) {
                    while (temp == "") {
                        getline (words, temp, ' ');
                    }

                    if (isdigit (temp[0])) {
                        data[l++].S = stoi (temp);
                    }
                }
            }
        }

        input.close();
    }

    ll p = 1;

    for (size_t i = 0; i < data.size(); i++) {
        int a = data[i].F;
        int b = data[i].S;
        int j = 0;

        for (j = 1; j <= b / 2; j++) {
            int d = (j) * (a - j);

            if (d > b) {
                break;
            }
        }

        int one = j;
        int two = a - j;
        p *= (two - one + 1);
    }

    cout << p;
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
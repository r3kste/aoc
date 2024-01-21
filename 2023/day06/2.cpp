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

    ll time = 0;
    ll dist = 0;
    string tt = "";
    string td = "";

    for (size_t i = 0; i < data.size(); i++) {
        tt += (to_string (data[i].F));
        td += (to_string (data[i].S));
    }

    time = stoll (tt);
    dist = stoll (td);
    ll j = 0;

    for (j = time / 2; j >= 1; j--) {
        ll d = (j) * (time - j);

        if (d < dist) {
            break;
        }
    }

    ll one = j;
    ll two = time - j;
    cout << abs (two - one) - 1;
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
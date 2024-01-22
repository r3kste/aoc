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
    std::filesystem::path path (__FILE__);
    path = path.parent_path();
    path /= "input.txt";
    input.open (path);
    string line;
    vii data (4);
    int k = 0;
    int l = 0;

    if (input.is_open()) {
        while (getline (input, line)) {
            stringstream words (line);
            string temp;
            getline (words, temp, ' ');

            if (temp == "Time:") {
                while (getline (words, temp, ' ')) {
                    while (temp.empty()) {
                        getline (words, temp, ' ');
                    }

                    if (isdigit (temp[0])) {
                        data[k++].F = stoi (temp);
                    }
                }
            } else if (temp == "Distance:") {
                while (getline (words, temp, ' ')) {
                    while (temp.empty()) {
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

    ll time;
    ll dist;
    string tt;
    string td;

    for (auto &i : data) {
        tt += (to_string (i.F));
        td += (to_string (i.S));
    }

    time = stoll (tt);
    dist = stoll (td);
    ll j;

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
    fastio
    int t = 1;

    while (t--) {
        solve();
        cout << "\n";
    }
}
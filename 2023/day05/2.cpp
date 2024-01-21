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
    string file = "input.txt";
    input.open (file);
    string line;
    ll mapto;
    getline (input, line);
    stringstream words (line);
    string tempo1, tempo2;
    vector < pair < ll, ll>> seeds;
    map<ll, vector<vll>> p;

    while (getline (words, tempo1, ' ')) {
        if (isdigit (tempo1[0])) {
            getline (words, tempo2, ' ');
            ll t1 = stoll (tempo1);
            ll t2 = stoll (tempo2);
            seeds.pb (mp (t1, t2 + t1));
        }
    }

    if (input.is_open()) {
        while ( getline (input, line) ) {
            if (line == "") {
                continue;
            } else {
                if (line == "seed-to-soil map:") {
                    mapto = 1;
                } else if (line == "soil-to-fertilizer map:") {
                    mapto = 2;
                } else if (line == "fertilizer-to-water map:") {
                    mapto = 3;
                } else if (line == "water-to-light map:") {
                    mapto = 4;
                } else if (line == "light-to-temperature map:") {
                    mapto = 5;
                } else if (line == "temperature-to-humidity map:") {
                    mapto = 6;
                } else if (line == "humidity-to-location map:") {
                    mapto = 7;
                } else {
                    stringstream indices (line);
                    string temp;
                    vll ttt;

                    while (getline (indices, temp, ' ')) {
                        if (isdigit (temp[0])) {
                            ttt.pb (stoll (temp));
                        }
                    }

                    if (ttt.size() == 3) {
                        p[mapto - 1].pb ({ttt[0], ttt[1], ttt[2]});
                    }
                }
            }
        }

        input.close();
    }

    for (auto operation : p) {
        vector<pair<ll, ll>> next;

        while (seeds.size() > 0) {
            ll l = seeds.back().F;
            ll r = seeds.back().S;
            seeds.pop_back();
            bool flag;

            for (auto limits : operation.S) {
                ll dest_start = limits[0];
                ll sauc_start = limits[1];
                ll lent_lenth = limits[2];
                ll os = max (l, sauc_start);
                ll oe = min (r, sauc_start + lent_lenth);
                flag = false;

                if (os < oe) {
                    next.pb (mp (os - sauc_start + dest_start, oe - sauc_start + dest_start));

                    if (os > l) {
                        seeds.pb (mp (l, os));
                    }

                    if (oe < r) {
                        seeds.pb (mp (oe, r));
                    }

                    flag = true;
                    break;
                }
            }

            if (!flag) {
                next.pb (mp (l, r));
            }
        }

        seeds = next;
    }

    ll min = MOD * MOD;

    for (auto i : seeds) {
        if (i.F < min) {
            min = i.F;
        }
    }

    cout << min;
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

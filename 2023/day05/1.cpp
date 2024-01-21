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
    ll mapto = 0;
    getline (input, line);
    stringstream words (line);
    string tempo;
    vll seeds;
    map<ll, vll> p;

    while (getline (words, tempo, ' ')) {
        if (isdigit (tempo[0])) {
            seeds.pb (stoll (tempo));
        }
    }

    input.close();

    for (size_t i = 0; i < seeds.size(); i++) {
        for (int j = 0; j < 7; j++) {
            p[seeds[i]].pb (seeds[i]);
        }
    }

    // bool flag = true;

    for (size_t i = 0; i < seeds.size(); i++) {
        input.open (file);
        getline (input, line);
        ll mapf = 0;
        // int mapt=0;

        if (input.is_open()) {
            while ( getline (input, line) ) {
                if (line == "") {
                    continue;
                } else {
                    if (line == "seed-to-soil map:") {
                        mapf = seeds[i];
                        mapto = 1;
                        p[seeds[i]][mapto - 1] = mapf;
                    } else if (line == "soil-to-fertilizer map:") {
                        mapf = p[seeds[i]][0];
                        mapto = 2;
                        p[seeds[i]][mapto - 1] = mapf;
                    } else if (line == "fertilizer-to-water map:") {
                        mapf = p[seeds[i]][1];
                        mapto = 3;
                        p[seeds[i]][mapto - 1] = mapf;
                    } else if (line == "water-to-light map:") {
                        mapf = p[seeds[i]][2];
                        mapto = 4;
                        p[seeds[i]][mapto - 1] = mapf;
                    } else if (line == "light-to-temperature map:") {
                        mapf = p[seeds[i]][3];
                        mapto = 5;
                        p[seeds[i]][mapto - 1] = mapf;
                    } else if (line == "temperature-to-humidity map:") {
                        mapf = p[seeds[i]][4];
                        mapto = 6;
                        p[seeds[i]][mapto - 1] = mapf;
                    } else if (line == "humidity-to-location map:") {
                        mapf = p[seeds[i]][5];
                        mapto = 7;
                        p[seeds[i]][mapto - 1] = mapf;
                    } else {
                        stringstream indices (line);
                        string temp;
                        vll tindex;

                        while (getline (indices, temp, ' ')) {
                            if (isdigit (temp[0])) {
                                tindex.pb (stoll (temp));
                            }
                        }

                        if (mapf >= tindex[1] && mapf <= (tindex[2] + tindex[1] - 1)) {
                            p[seeds[i]][mapto - 1] = mapf - tindex[1] + tindex[0];
                        }
                    }
                }
            }

            input.close();
        }
    }

    ll min = MOD * MOD;

    for (size_t i = 0; i < seeds.size(); i++) {
        if (p[seeds[i]][6] < min) {
            min = p[seeds[i]][6];
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

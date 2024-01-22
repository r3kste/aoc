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

    for (long long seed : seeds) {
        for (int j = 0; j < 7; j++) {
            p[seed].pb (seed);
        }
    }

    // bool flag = true;

    for (long long seed : seeds) {
        input.open (path);
        getline (input, line);
        ll mapf = 0;
        // int mapt=0;

        if (input.is_open()) {
            while (getline (input, line)) {
                if (line.empty()) {
                    continue;
                } else {
                    if (line == "seed-to-soil map:") {
                        mapf = seed;
                        mapto = 1;
                        p[seed][mapto - 1] = mapf;
                    } else if (line == "soil-to-fertilizer map:") {
                        mapf = p[seed][0];
                        mapto = 2;
                        p[seed][mapto - 1] = mapf;
                    } else if (line == "fertilizer-to-water map:") {
                        mapf = p[seed][1];
                        mapto = 3;
                        p[seed][mapto - 1] = mapf;
                    } else if (line == "water-to-light map:") {
                        mapf = p[seed][2];
                        mapto = 4;
                        p[seed][mapto - 1] = mapf;
                    } else if (line == "light-to-temperature map:") {
                        mapf = p[seed][3];
                        mapto = 5;
                        p[seed][mapto - 1] = mapf;
                    } else if (line == "temperature-to-humidity map:") {
                        mapf = p[seed][4];
                        mapto = 6;
                        p[seed][mapto - 1] = mapf;
                    } else if (line == "humidity-to-location map:") {
                        mapf = p[seed][5];
                        mapto = 7;
                        p[seed][mapto - 1] = mapf;
                    } else {
                        stringstream indices (line);
                        string temp;
                        vll temp_index;

                        while (getline (indices, temp, ' ')) {
                            if (isdigit (temp[0])) {
                                temp_index.pb (stoll (temp));
                            }
                        }

                        if (mapf >= temp_index[1] && mapf <= (temp_index[2] + temp_index[1] - 1)) {
                            p[seed][mapto - 1] = mapf - temp_index[1] + temp_index[0];
                        }
                    }
                }
            }

            input.close();
        }
    }

    ll min = MOD * MOD;

    for (long long seed : seeds) {
        if (p[seed][6] < min) {
            min = p[seed][6];
        }
    }

    cout << min;
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

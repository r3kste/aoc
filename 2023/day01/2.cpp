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
    ll sum = 0;
    vector<pair<string, string>> e = {{"one",   "o1e"},
                                      {"two",   "t2o"},
                                      {"three", "t3e"},
                                      {"four",  "f4r"},
                                      {"five",  "f5e"},
                                      {"six",   "s6x"},
                                      {"seven", "s7n"},
                                      {"eight", "e8t"},
                                      {"nine",  "n9e"}};

    if (input.is_open()) {
        while (getline(input, line)) {
            for (auto &i: e) {
                while (line.find(i.F) != string::npos) {
                    line.replace(line.find(i.F), i.F.size(), i.S);
                }
            }

            for (auto &i: e) {
                while (line.find(i.F) != string::npos) {
                    line.replace(line.find(i.F), i.F.size(), i.S);
                }
            }

            for (auto &i: e) {
                while (line.find(i.F) != string::npos) {
                    line.replace(line.find(i.F), i.F.size(), i.S);
                }
            }

            int a = 0;
            int b = 0;
            bool flag = true;

            for (char i: line) {
                if (int(i) >= 48 && int(i) <= 57) {
                    if (flag) {
                        a = int(i) - 48;
                        b = a;
                        flag = false;
                    } else {
                        b = int(i) - 48;
                    }
                }
            }

            sum += (10 * a) + b;
        }

        input.close();
    }

    cout << sum;
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

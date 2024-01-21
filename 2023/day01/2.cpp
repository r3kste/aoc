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
    ll sum = 0;
    // vector<string>num {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    vector<pair<string, string>> e = {{"one", "o1e"}, {"two", "t2o"}, {"three", "t3e"}, {"four", "f4r"}, {"five", "f5e"}, {"six", "s6x"}, {"seven", "s7n"}, {"eight", "e8t"}, {"nine", "n9e"}};

    if (input.is_open()) {
        while ( getline (input, line) ) {
            for (size_t i = 0; i < e.size(); i++) {
                while (line.find (e[i].F) != string::npos) {
                    line.replace (line.find (e[i].F), e[i].F.size(), e[i].S);
                }
            }

            for (size_t i = 0; i < e.size(); i++) {
                while (line.find (e[i].F) != string::npos) {
                    line.replace (line.find (e[i].F), e[i].F.size(), e[i].S);
                }
            }

            for (size_t i = 0; i < e.size(); i++) {
                while (line.find (e[i].F) != string::npos) {
                    line.replace (line.find (e[i].F), e[i].F.size(), e[i].S);
                }
            }

            int a = 0;
            int b = 0;
            bool flag = true;

            for (size_t i = 0; i < line.size(); i++) {
                if (int (line[i]) >= 48 && int (line[i]) <= 57) {
                    if (flag) {
                        a = int (line[i]) - 48;
                        b = a;
                        flag = false;
                    } else {
                        b = int (line[i]) - 48;
                    }
                }
            }

            sum += (10 * a) + b;
            // int a = int (MOD);
            // int b = -1;
            // bool flag = true;
            // vi existence_lower (10, -1);
            // vi existence_upper (10, -1);
            // line = "two1two";
            // for (int i = 0; i < 10; i++)
            // {
            //     // if (flag)
            //     // {
            //     //     existence_lower[i] = line.find (num[i]);
            //     //     existence_upper[i] = line.find (num[i]);
            //     //     if (line.find (num[i]) != -1)
            //     //     {
            //     //         flag = false;
            //     //     }
            //     // }
            //     // else
            //     // {
            //     //     existence_upper[i] = line.find (num[i]);
            //     // }
            //     if (line.find (num[i]) != string::npos)
            //     {
            //         existence_lower[i] = line.find (num[i]);
            //         int wtf = line.find (num[i].res) - num[i].size() + 1;
            //         if (line.find_last_of (num[i]) < line.size() - 1)
            //         {
            //             existence_upper[i] = wtf;
            //         }
            //     }
            // }
            // flag = true;
            // for (int i = 0; i < line.size(); i++)
            // {
            //     if (int (line[i]) >= 48 && int (line[i]) <= 57)
            //     {
            //         if (flag)
            //         {
            //             if (existence_lower[ (line[i]) - 48] == -1)
            //             {
            //                 existence_lower[ (line[i]) - 48] = i;
            //                 existence_upper[ (line[i]) - 48] = i;
            //             }
            //             else
            //             {
            //                 existence_lower[ (line[i]) - 48] = min (i, existence_lower[ (line[i]) - 48]);
            //                 existence_upper[ (line[i]) - 48]  = min (i, existence_lower[ (line[i]) - 48]);
            //             }
            //             flag = false;
            //         }
            //         else
            //         {
            //             if (existence_upper[ (line[i]) - 48] == -1)
            //             {
            //                 existence_upper[ (line[i]) - 48] = i;
            //             }
            //             else
            //             {
            //                 existence_upper[ (line[i]) - 48] = max (i, existence_upper[ (line[i]) - 48]);
            //             }
            //         }
            //     }
            // }
            // int maxt = int (-MOD);
            // int mint = int (MOD);
            // for (int i = 0; i < 10; i++)
            // {
            //     if (existence_lower[i] != -1 && existence_lower[i] < mint)
            //     {
            //         a = i;
            //         mint = existence_lower[a];
            //     }
            //     if (existence_upper[i] > maxt)
            //     {
            //         b = i;
            //         maxt = existence_upper[b];
            //     }
            // }
            // // a = existence_lower[a];
            // // b = existence_upper[b];
            // sum += (10 * a) + b;
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

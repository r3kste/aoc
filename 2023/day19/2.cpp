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

typedef unsigned long long int ll;
typedef unsigned long long int LL;
typedef pair<int, int> ii;

typedef vector<int> vi;
typedef vector<pair<int, int>> vii;
typedef vector<long long int> vll;

map < string, vector<pair<pair<pair<char, char>, int>, string>>> p;
map<string, LL> dp;

LL ff (string workflow, map<char, pair<int, int>> range) {
    if (workflow == "A") {
        ll ans = 1;

        for (auto i : range) {
            int l = i.S.F;
            int r = i.S.S;
            ans *= (r - l + 1);
        }

        return ans;
    } else if (workflow == "R") {
        return 0;
    } else {
        ll sum = 0;
        map<char, pair<int, int>> xmas = range;
        ii t;
        ii f;
        bool flag = true;
        string target;
        char quantity, ineq;

        for (auto i : p[workflow]) {
            flag = true;
            target = i.S;
            quantity = i.F.F.F;
            ineq = i.F.F.S;
            int n = i.F.S;
            int l = xmas[quantity].F;
            int r = xmas[quantity].S;

            switch (ineq) {
            case '<':
                t = mp (l, n - 1);
                f = mp (n, r);
                break;

            case '>':
                t = mp (n + 1, r);
                f = mp (l, n);
                break;

            case ' ':
                goto oo;

            default:
                break;
            }

            if (t.F <= t.S) {
                auto copy = xmas;
                copy[quantity] = t;
                sum += ff (target, copy);
            }

            if (f.F <= f.S) {
                xmas[quantity] = f; //change the limits and go to next rule
            } else {
                flag = false;
                break; //there is no false case, so just end it
            }
        }

        if (flag) {
oo:
            sum += ff (target, xmas);
        }

        return sum;
    }
}
int solve() {
    fastio;
    ifstream input;
    input.open ("input.txt");
    string line;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            if (line == "") {
                getline (input, line);
                break;
            }

            string workflow = line.substr (0, line.find ('{'));
            line = line.substr (line.find ('{') + 1, line.find ('}') - workflow.size() - 1);
            stringstream operations (line);
            string operation;

            while (getline (operations, operation, ',')) {
                char quantity, ineq;
                string num, target;
                ll number;

                if (operation.size() > 1 && isdigit (operation[2])) {
                    quantity = operation[0];
                    ineq = operation[1];
                    num = operation.substr (2, operation.find (':') - 2);
                    number = stoll (num);
                    target = operation.substr (operation.find (':') + 1, operation.size() - 3 - num.size());
                } else {
                    quantity = ' ';
                    ineq = ' ';
                    number = 0;
                    target = operation;
                }

                p[workflow].pb (mp (mp (mp (quantity, ineq), number), target));
            }
        }

        input.close();
    }

    // f(wf,ranges) = number of ways to go from that workflow to 'A' with ranges as limits
    map<char, pair<int, int>> xmas;
    xmas['x'] = mp (1, 4000);
    xmas['m'] = mp (1, 4000);
    xmas['a'] = mp (1, 4000);
    xmas['s'] = mp (1, 4000);
    cout << ff ("in", xmas);
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
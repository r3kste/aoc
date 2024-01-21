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
    bool flag = true;
    map < string, vector<pair<pair<pair<char, char>, int>, string>>> p;
    //quantity,ineq,number,target
    // vector<map<char, int>> q;
    ll ans = 0;

    if (input.is_open()) {
        while ( getline (input, line) ) {
            if (line == "") {
                getline (input, line);
                flag = false;
            }

            if (flag) {
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
            } else {
                line = line.substr (1, line.size() - 2);
                stringstream data (line);
                string detes;
                char quantity, ineq;
                string num, target, workflow;
                ll number, sum = 0;
                map<char, int> q;

                while (getline (data, detes, ',')) {
                    quantity = detes[0];
                    string num = detes.substr (2, detes.size() - 2);
                    number = stoll (num);
                    sum += number;
                    q[quantity] = number;
                }

                workflow = "in";

                // j is operation in workflow
                for (size_t j = 0; j < p[workflow].size(); j++) {
                    auto i = p[workflow][j];
                    quantity = i.F.F.F;
                    ineq = i.F.F.S;
                    number = i.F.S;
                    target = i.S;

                    switch (ineq) {
                    case '<':
                        if (q[quantity] < number) {
                            if (target == "A") {
                                ans += sum;
                                j = p[workflow].size();
                            } else if (target == "R") {
                                j = p[workflow].size();
                            } else {
                                workflow = target;
                                j = -1;
                            }
                        }

                        break;

                    case '>':
                        if (q[quantity] > number) {
                            if (target == "A") {
                                ans += sum;
                                j = p[workflow].size();
                            } else if (target == "R") {
                                j = p[workflow].size();
                            } else {
                                workflow = target;
                                j = -1;
                            }
                        }

                        break;

                    case ' ':
                        if (target == "A") {
                            ans += sum;
                            j = p[workflow].size();
                        } else if (target == "R") {
                            j = p[workflow].size();
                        } else {
                            workflow = target;
                            j = -1;
                        }

                    default:
                        break;
                    }
                }
            }
        }

        input.close();
    }

    cout << ans;
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
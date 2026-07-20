// dice_cpp.cpp — генератор случайных чисел (игральные кости) на C++

#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <random>
#include <chrono>
#include <thread>
#include <map>
#include <fstream>
#include <iomanip>
#include <algorithm>
#include <cctype>

using namespace std;

struct RollEntry {
    time_t timestamp;
    string dice;
    vector<int> results;
    int total;
    bool weighted;
};

class DiceRoller {
private:
    vector<RollEntry> history;
    mt19937 rng;
    bool weightedMode;

public:
    DiceRoller() : rng(chrono::steady_clock::now().time_since_epoch().count()), weightedMode(false) {
        loadHistory();
    }

    void loadHistory() {
        ifstream file("dice_history.json");
        if (!file.is_open()) return;
        // Упрощённо: не парсим JSON, оставляем пустым
        file.close();
    }

    void saveHistory() {
        // Упрощённо: не сохраняем в JSON, только в память
    }

    vector<int> roll(int numDice, int numFaces, bool weighted = false) {
        vector<int> results;
        uniform_int_distribution<int> dist(1, numFaces);
        for (int i = 0; i < numDice; ++i) {
            if (weighted) {
                // Смещение: сдвиг в сторону больших значений (симуляция нечестной кости)
                double r = (double)rng() / rng.max();
                int val = (int)(r * r * numFaces) + 1; // квадратичное смещение
                if (val > numFaces) val = numFaces;
                results.push_back(val);
            } else {
                results.push_back(dist(rng));
            }
        }
        return results;
    }

    void animateRoll(int numDice, int numFaces, bool weighted) {
        cout << "Бросок" << (weighted ? " (взвешенный)" : "") << "..." << endl;
        for (int i = 0; i < 5; ++i) {
            cout << "\r🎲 ";
            for (int j = 0; j < numDice; ++j) {
                cout << (rand() % numFaces + 1) << " ";
            }
            cout.flush();
            this_thread::sleep_for(chrono::milliseconds(150));
        }
        vector<int> results = roll(numDice, numFaces, weighted);
        int total = 0;
        for (int v : results) total += v;
        cout << "\r🎲 Результат: ";
        for (size_t i = 0; i < results.size(); ++i) {
            if (i) cout << ", ";
            cout << results[i];
        }
        cout << " → Сумма: " << total << endl;

        RollEntry entry;
        entry.timestamp = time(nullptr);
        entry.dice = to_string(numDice) + "d" + to_string(numFaces);
        entry.results = results;
        entry.total = total;
        entry.weighted = weighted;
        history.push_back(entry);
        saveHistory();
    }

    void showStats() {
        if (history.empty()) {
            cout << "История пуста." << endl;
            return;
        }
        vector<int> totals;
        for (auto& e : history) totals.push_back(e.total);
        double avg = 0;
        for (int v : totals) avg += v;
        avg /= totals.size();
        sort(totals.begin(), totals.end());
        int median = totals[totals.size()/2];
        // Мода
        map<int, int> freq;
        for (int v : totals) freq[v]++;
        int mode = totals[0], maxFreq = 0;
        for (auto& p : freq) if (p.second > maxFreq) { maxFreq = p.second; mode = p.first; }
        cout << "Всего бросков: " << history.size() << endl;
        cout << "Среднее: " << fixed << setprecision(2) << avg << endl;
        cout << "Медиана: " << median << endl;
        cout << "Мода: " << mode << endl;
    }

    void showHistory() {
        if (history.empty()) {
            cout << "История пуста." << endl;
            return;
        }
        for (size_t i = 0; i < history.size(); ++i) {
            auto& e = history[i];
            cout << i+1 << ". " << e.dice;
            if (e.weighted) cout << " (взв.)";
            cout << " → ";
            for (size_t j = 0; j < e.results.size(); ++j) {
                if (j) cout << ",";
                cout << e.results[j];
            }
            cout << " (сумма " << e.total << ")" << endl;
        }
    }

    void clearHistory() {
        history.clear();
        saveHistory();
        cout << "История очищена." << endl;
    }

    bool parseRoll(const string& cmd, int& numDice, int& numFaces, bool& weighted) {
        istringstream iss(cmd);
        string token;
        iss >> token; // "roll"
        iss >> token; // "3d6"
        if (token.find('d') == string::npos) return false;
        string numDiceStr = token.substr(0, token.find('d'));
        string numFacesStr = token.substr(token.find('d') + 1);
        numDice = stoi(numDiceStr);
        numFaces = stoi(numFacesStr);
        weighted = (cmd.find("--weighted") != string::npos);
        return (numDice > 0 && numFaces > 0);
    }

    void run() {
        cout << "🎲 DiceMaster Pro — C++ Edition" << endl;
        cout << "Команды: roll NDМ [--weighted], history, stats, clear, exit" << endl;
        string cmd;
        while (true) {
            cout << "> ";
            getline(cin, cmd);
            if (cmd == "exit" || cmd == "quit") {
                cout << "До свидания!" << endl;
                break;
            } else if (cmd == "history") {
                showHistory();
            } else if (cmd == "stats") {
                showStats();
            } else if (cmd == "clear") {
                clearHistory();
            } else if (cmd.find("roll") == 0) {
                int numDice, numFaces;
                bool weighted;
                if (parseRoll(cmd, numDice, numFaces, weighted)) {
                    animateRoll(numDice, numFaces, weighted);
                } else {
                    cout << "Неверный формат. Используйте: roll 3d6 или roll 2d20 --weighted" << endl;
                }
            } else {
                cout << "Неизвестная команда" << endl;
            }
        }
    }
};

int main() {
    DiceRoller roller;
    roller.run();
    return 0;
}

// dice_js.js — генератор случайных чисел (игральные кости) на JavaScript (Node.js)

const readline = require('readline');
const fs = require('fs');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

class DiceRoller {
    constructor() {
        this.history = [];
        this.loadHistory();
    }

    loadHistory() {
        try {
            const data = fs.readFileSync('dice_history.json', 'utf8');
            this.history = JSON.parse(data);
        } catch (e) {
            this.history = [];
        }
    }

    saveHistory() {
        fs.writeFileSync('dice_history.json', JSON.stringify(this.history, null, 2));
    }

    roll(numDice, numFaces, weighted) {
        const results = [];
        for (let i = 0; i < numDice; i++) {
            let val;
            if (weighted) {
                const r = Math.random();
                val = Math.floor(r * r * numFaces) + 1;
                if (val > numFaces) val = numFaces;
            } else {
                val = Math.floor(Math.random() * numFaces) + 1;
            }
            results.push(val);
        }
        return results;
    }

    animateRoll(numDice, numFaces, weighted) {
        return new Promise((resolve) => {
            console.log(`Бросок${weighted ? ' (взвешенный)' : ''}...`);
            let count = 0;
            const interval = setInterval(() => {
                process.stdout.write('\r🎲 ');
                for (let i = 0; i < numDice; i++) {
                    process.stdout.write(`${Math.floor(Math.random() * numFaces) + 1} `);
                }
                count++;
                if (count >= 5) {
                    clearInterval(interval);
                    const results = this.roll(numDice, numFaces, weighted);
                    const total = results.reduce((a, b) => a + b, 0);
                    console.log(`\r🎲 Результат: ${results.join(', ')} → Сумма: ${total}`);
                    this.history.push({
                        timestamp: Date.now(),
                        dice: `${numDice}d${numFaces}`,
                        results,
                        total,
                        weighted
                    });
                    this.saveHistory();
                    resolve();
                }
            }, 150);
        });
    }

    showStats() {
        if (this.history.length === 0) {
            console.log('История пуста.');
            return;
        }
        const totals = this.history.map(e => e.total);
        const sum = totals.reduce((a, b) => a + b, 0);
        const avg = sum / totals.length;
        const sorted = [...totals].sort((a, b) => a - b);
        const median = sorted[Math.floor(sorted.length / 2)];
        const freq = {};
        for (const v of totals) freq[v] = (freq[v] || 0) + 1;
        let mode = totals[0], maxFreq = 0;
        for (const [k, v] of Object.entries(freq)) {
            if (v > maxFreq) { maxFreq = v; mode = Number(k); }
        }
        console.log(`Всего бросков: ${this.history.length}`);
        console.log(`Среднее: ${avg.toFixed(2)}`);
        console.log(`Медиана: ${median}`);
        console.log(`Мода: ${mode}`);
    }

    showHistory() {
        if (this.history.length === 0) {
            console.log('История пуста.');
            return;
        }
        for (let i = 0; i < this.history.length; i++) {
            const e = this.history[i];
            process.stdout.write(`${i+1}. ${e.dice}`);
            if (e.weighted) process.stdout.write(' (взв.)');
            console.log(` → ${e.results.join(',')} (сумма ${e.total})`);
        }
    }

    clearHistory() {
        this.history = [];
        this.saveHistory();
        console.log('История очищена.');
    }

    parseRoll(cmd) {
        const parts = cmd.split(' ');
        if (parts.length < 2) return null;
        const diceStr = parts[1];
        if (!diceStr.includes('d')) return null;
        const [numDice, numFaces] = diceStr.split('d').map(Number);
        if (isNaN(numDice) || isNaN(numFaces) || numDice <= 0 || numFaces <= 0) return null;
        const weighted = cmd.includes('--weighted');
        return { numDice, numFaces, weighted };
    }

    async run() {
        console.log('🎲 DiceMaster Pro — JavaScript Edition');
        console.log('Команды: roll NDМ [--weighted], history, stats, clear, exit');
        const ask = () => {
            rl.question('> ', async (cmd) => {
                cmd = cmd.trim().toLowerCase();
                if (cmd === 'exit' || cmd === 'quit') {
                    console.log('До свидания!');
                    rl.close();
                    return;
                } else if (cmd === 'history') {
                    this.showHistory();
                } else if (cmd === 'stats') {
                    this.showStats();
                } else if (cmd === 'clear') {
                    this.clearHistory();
                } else if (cmd.startsWith('roll')) {
                    const parsed = this.parseRoll(cmd);
                    if (parsed) {
                        await this.animateRoll(parsed.numDice, parsed.numFaces, parsed.weighted);
                    } else {
                        console.log('Неверный формат. Используйте: roll 3d6 или roll 2d20 --weighted');
                    }
                } else {
                    console.log('Неизвестная команда');
                }
                ask();
            });
        };
        ask();
    }
}

const roller = new DiceRoller();
roller.run();

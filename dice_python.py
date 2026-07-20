# dice_python.py — генератор случайных чисел (игральные кости) на Python

import random
import time
import json
import os
import sys
from collections import Counter, defaultdict

class DiceRoller:
    def __init__(self):
        self.history = []
        self.load_history()
        self.weighted_mode = False
        self.colors = {'red': '\033[91m', 'green': '\033[92m', 'yellow': '\033[93m', 'blue': '\033[94m', 'reset': '\033[0m'}

    def load_history(self):
        if os.path.exists('dice_history.json'):
            with open('dice_history.json', 'r') as f:
                self.history = json.load(f)

    def save_history(self):
        with open('dice_history.json', 'w') as f:
            json.dump(self.history, f, indent=2)

    def roll(self, num_dice, num_faces, weighted=False):
        """Бросок костей. Если weighted=True, используем смещённые вероятности."""
        if weighted:
            # Создаём смещённое распределение: увеличиваем вероятность высоких значений
            weights = [i for i in range(1, num_faces+1)]  # линейное смещение
            results = [random.choices(range(1, num_faces+1), weights=weights, k=1)[0] for _ in range(num_dice)]
        else:
            results = [random.randint(1, num_faces) for _ in range(num_dice)]
        total = sum(results)
        entry = {
            'time': time.time(),
            'dice': f"{num_dice}d{num_faces}",
            'results': results,
            'total': total,
            'weighted': weighted
        }
        self.history.append(entry)
        self.save_history()
        return results, total

    def animate_roll(self, num_dice, num_faces, weighted=False):
        """Анимация броска с задержкой."""
        print("Бросок" + (" (взвешенный)" if weighted else "") + "...")
        for i in range(5):
            sys.stdout.write('\r' + '🎲 ' + ' '.join([str(random.randint(1, num_faces)) for _ in range(num_dice)]))
            sys.stdout.flush()
            time.sleep(0.15)
        results, total = self.roll(num_dice, num_faces, weighted)
        print('\r' + ' ' * 30, end='')
        print(f"\r🎲 Результат: {', '.join(map(str, results))} → {self.colors['green']}Сумма: {total}{self.colors['reset']}")
        return results, total

    def show_stats(self):
        if not self.history:
            print("История пуста.")
            return
        totals = [entry['total'] for entry in self.history]
        avg = sum(totals) / len(totals)
        median = sorted(totals)[len(totals)//2]
        mode = Counter(totals).most_common(1)[0][0] if totals else None
        print(f"Всего бросков: {len(self.history)}")
        print(f"Среднее: {avg:.2f}")
        print(f"Медиана: {median}")
        if mode:
            print(f"Мода: {mode}")
        # Распределение сумм
        dist = Counter(totals)
        print("Распределение сумм:")
        for val, count in sorted(dist.items()):
            bar = '█' * min(count, 20)
            print(f"  {val:3d}: {bar} ({count})")

    def show_history(self):
        if not self.history:
            print("История пуста.")
            return
        for i, entry in enumerate(self.history, 1):
            results_str = ','.join(map(str, entry['results']))
            weighted_str = " (взв.)" if entry.get('weighted', False) else ""
            print(f"{i:2d}. {entry['dice']}{weighted_str} → {results_str} (сумма {entry['total']})")

    def clear_history(self):
        self.history = []
        self.save_history()
        print("История очищена.")

    def parse_roll(self, cmd):
        """Парсинг команды вида 'roll 3d6' или 'roll 2d20 --weighted'"""
        parts = cmd.split()
        if len(parts) < 2:
            return None
        dice_str = parts[1]
        weighted = '--weighted' in parts
        if 'd' not in dice_str:
            return None
        try:
            num_dice, num_faces = map(int, dice_str.split('d'))
            if num_dice <= 0 or num_faces <= 0:
                return None
            return num_dice, num_faces, weighted
        except ValueError:
            return None

    def run(self):
        print("🎲 DiceMaster Pro — Python Edition")
        print("Команды: roll NDМ [--weighted], history, stats, clear, exit")
        while True:
            cmd = input("> ").strip().lower()
            if cmd in ('exit', 'quit'):
                print("До свидания!")
                break
            elif cmd == 'history':
                self.show_history()
            elif cmd == 'stats':
                self.show_stats()
            elif cmd == 'clear':
                self.clear_history()
            elif cmd.startswith('roll'):
                parsed = self.parse_roll(cmd)
                if parsed:
                    num_dice, num_faces, weighted = parsed
                    self.animate_roll(num_dice, num_faces, weighted)
                else:
                    print("Неверный формат. Используйте: roll 3d6 или roll 2d20 --weighted")
            else:
                print("Неизвестная команда")

if __name__ == "__main__":
    roller = DiceRoller()
    roller.run()

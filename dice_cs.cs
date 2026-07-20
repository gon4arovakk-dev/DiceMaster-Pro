// dice_cs.cs — генератор случайных чисел (игральные кости) на C#

using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

class DiceRoller
{
    private class RollEntry
    {
        public long Timestamp { get; set; }
        public string Dice { get; set; }
        public List<int> Results { get; set; }
        public int Total { get; set; }
        public bool Weighted { get; set; }
    }

    private List<RollEntry> history = new List<RollEntry>();
    private Random rand = new Random();

    public DiceRoller()
    {
        // Загрузка истории не реализована для простоты
    }

    private void SaveHistory() { }

    public List<int> Roll(int numDice, int numFaces, bool weighted)
    {
        var results = new List<int>();
        for (int i = 0; i < numDice; i++)
        {
            int val;
            if (weighted)
            {
                double r = rand.NextDouble();
                val = (int)(Math.Pow(r, 0.5) * numFaces) + 1;
                if (val > numFaces) val = numFaces;
            }
            else
            {
                val = rand.Next(1, numFaces + 1);
            }
            results.Add(val);
        }
        return results;
    }

    public async Task AnimateRoll(int numDice, int numFaces, bool weighted)
    {
        Console.WriteLine("Бросок" + (weighted ? " (взвешенный)" : "") + "...");
        for (int i = 0; i < 5; i++)
        {
            Console.Write("\r🎲 ");
            for (int j = 0; j < numDice; j++)
                Console.Write((rand.Next(1, numFaces + 1)) + " ");
            await Task.Delay(150);
        }
        var results = Roll(numDice, numFaces, weighted);
        int total = results.Sum();
        Console.Write("\r🎲 Результат: ");
        Console.Write(string.Join(", ", results));
        Console.WriteLine($" → Сумма: {total}");

        var entry = new RollEntry
        {
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
            Dice = $"{numDice}d{numFaces}",
            Results = results,
            Total = total,
            Weighted = weighted
        };
        history.Add(entry);
        SaveHistory();
    }

    public void ShowStats()
    {
        if (!history.Any())
        {
            Console.WriteLine("История пуста.");
            return;
        }
        var totals = history.Select(e => e.Total).ToList();
        double avg = totals.Average();
        totals.Sort();
        int median = totals[totals.Count / 2];
        var mode = totals.GroupBy(v => v).OrderByDescending(g => g.Count()).First().Key;
        Console.WriteLine($"Всего бросков: {history.Count}");
        Console.WriteLine($"Среднее: {avg:F2}");
        Console.WriteLine($"Медиана: {median}");
        Console.WriteLine($"Мода: {mode}");
    }

    public void ShowHistory()
    {
        if (!history.Any())
        {
            Console.WriteLine("История пуста.");
            return;
        }
        for (int i = 0; i < history.Count; i++)
        {
            var e = history[i];
            Console.Write($"{i+1}. {e.Dice}");
            if (e.Weighted) Console.Write(" (взв.)");
            Console.Write($" → {string.Join(",", e.Results)} (сумма {e.Total})");
            Console.WriteLine();
        }
    }

    public void ClearHistory()
    {
        history.Clear();
        SaveHistory();
        Console.WriteLine("История очищена.");
    }

    public bool ParseRoll(string cmd, out int numDice, out int numFaces, out bool weighted)
    {
        numDice = 0; numFaces = 0; weighted = false;
        var parts = cmd.Split(' ');
        if (parts.Length < 2) return false;
        var diceStr = parts[1];
        if (!diceStr.Contains('d')) return false;
        var nums = diceStr.Split('d');
        if (!int.TryParse(nums[0], out numDice) || !int.TryParse(nums[1], out numFaces))
            return false;
        weighted = cmd.Contains("--weighted");
        return numDice > 0 && numFaces > 0;
    }

    public async Task Run()
    {
        Console.WriteLine("🎲 DiceMaster Pro — C# Edition");
        Console.WriteLine("Команды: roll NDМ [--weighted], history, stats, clear, exit");
        while (true)
        {
            Console.Write("> ");
            string cmd = Console.ReadLine()?.Trim().ToLower() ?? "";
            if (cmd == "exit" || cmd == "quit")
            {
                Console.WriteLine("До свидания!");
                break;
            }
            else if (cmd == "history")
            {
                ShowHistory();
            }
            else if (cmd == "stats")
            {
                ShowStats();
            }
            else if (cmd == "clear")
            {
                ClearHistory();
            }
            else if (cmd.StartsWith("roll"))
            {
                if (ParseRoll(cmd, out int numDice, out int numFaces, out bool weighted))
                {
                    await AnimateRoll(numDice, numFaces, weighted);
                }
                else
                {
                    Console.WriteLine("Неверный формат. Используйте: roll 3d6 или roll 2d20 --weighted");
                }
            }
            else
            {
                Console.WriteLine("Неизвестная команда");
            }
        }
    }

    public static async Task Main(string[] args)
    {
        await new DiceRoller().Run();
    }
}

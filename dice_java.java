// dice_java.java — генератор случайных чисел (игральные кости) на Java

import java.util.*;
import java.io.*;
import java.time.*;

public class DiceRoller {
    private static class RollEntry {
        long timestamp;
        String dice;
        List<Integer> results;
        int total;
        boolean weighted;
    }

    private List<RollEntry> history = new ArrayList<>();
    private Random rand = new Random();

    public DiceRoller() {
        loadHistory();
    }

    private void loadHistory() {
        // Упрощённо: не загружаем из файла
    }

    private void saveHistory() {
        // Не сохраняем
    }

    public List<Integer> roll(int numDice, int numFaces, boolean weighted) {
        List<Integer> results = new ArrayList<>();
        for (int i = 0; i < numDice; i++) {
            int val;
            if (weighted) {
                // Смещение: больше вероятность высоких значений
                double r = rand.nextDouble();
                val = (int)(Math.pow(r, 0.5) * numFaces) + 1;
                if (val > numFaces) val = numFaces;
            } else {
                val = rand.nextInt(numFaces) + 1;
            }
            results.add(val);
        }
        return results;
    }

    public void animateRoll(int numDice, int numFaces, boolean weighted) {
        System.out.println("Бросок" + (weighted ? " (взвешенный)" : "") + "...");
        for (int i = 0; i < 5; i++) {
            System.out.print("\r🎲 ");
            for (int j = 0; j < numDice; j++) {
                System.out.print((rand.nextInt(numFaces) + 1) + " ");
            }
            System.out.flush();
            try { Thread.sleep(150); } catch (InterruptedException e) {}
        }
        List<Integer> results = roll(numDice, numFaces, weighted);
        int total = results.stream().mapToInt(Integer::intValue).sum();
        System.out.print("\r🎲 Результат: ");
        for (int i = 0; i < results.size(); i++) {
            if (i > 0) System.out.print(", ");
            System.out.print(results.get(i));
        }
        System.out.println(" → Сумма: " + total);

        RollEntry entry = new RollEntry();
        entry.timestamp = System.currentTimeMillis() / 1000;
        entry.dice = numDice + "d" + numFaces;
        entry.results = results;
        entry.total = total;
        entry.weighted = weighted;
        history.add(entry);
        saveHistory();
    }

    public void showStats() {
        if (history.isEmpty()) {
            System.out.println("История пуста.");
            return;
        }
        List<Integer> totals = new ArrayList<>();
        for (RollEntry e : history) totals.add(e.total);
        double avg = totals.stream().mapToInt(Integer::intValue).average().orElse(0);
        Collections.sort(totals);
        int median = totals.get(totals.size()/2);
        // Мода
        Map<Integer, Integer> freq = new HashMap<>();
        for (int v : totals) freq.put(v, freq.getOrDefault(v, 0) + 1);
        int mode = totals.get(0);
        int maxFreq = 0;
        for (Map.Entry<Integer, Integer> e : freq.entrySet()) {
            if (e.getValue() > maxFreq) { maxFreq = e.getValue(); mode = e.getKey(); }
        }
        System.out.println("Всего бросков: " + history.size());
        System.out.printf("Среднее: %.2f\n", avg);
        System.out.println("Медиана: " + median);
        System.out.println("Мода: " + mode);
    }

    public void showHistory() {
        if (history.isEmpty()) {
            System.out.println("История пуста.");
            return;
        }
        for (int i = 0; i < history.size(); i++) {
            RollEntry e = history.get(i);
            System.out.print((i+1) + ". " + e.dice);
            if (e.weighted) System.out.print(" (взв.)");
            System.out.print(" → ");
            for (int j = 0; j < e.results.size(); j++) {
                if (j > 0) System.out.print(",");
                System.out.print(e.results.get(j));
            }
            System.out.println(" (сумма " + e.total + ")");
        }
    }

    public void clearHistory() {
        history.clear();
        saveHistory();
        System.out.println("История очищена.");
    }

    public boolean parseRoll(String cmd, int[] params) {
        // params: [0]=numDice, [1]=numFaces, [2]=weighted (0/1)
        String[] parts = cmd.split("\\s+");
        if (parts.length < 2) return false;
        String diceStr = parts[1];
        if (!diceStr.contains("d")) return false;
        String[] nums = diceStr.split("d");
        try {
            params[0] = Integer.parseInt(nums[0]);
            params[1] = Integer.parseInt(nums[1]);
            params[2] = (cmd.contains("--weighted")) ? 1 : 0;
            return params[0] > 0 && params[1] > 0;
        } catch (NumberFormatException e) {
            return false;
        }
    }

    public void run() {
        System.out.println("🎲 DiceMaster Pro — Java Edition");
        System.out.println("Команды: roll NDМ [--weighted], history, stats, clear, exit");
        Scanner scanner = new Scanner(System.in);
        while (true) {
            System.out.print("> ");
            String cmd = scanner.nextLine().trim().toLowerCase();
            if (cmd.equals("exit") || cmd.equals("quit")) {
                System.out.println("До свидания!");
                break;
            } else if (cmd.equals("history")) {
                showHistory();
            } else if (cmd.equals("stats")) {
                showStats();
            } else if (cmd.equals("clear")) {
                clearHistory();
            } else if (cmd.startsWith("roll")) {
                int[] params = new int[3];
                if (parseRoll(cmd, params)) {
                    animateRoll(params[0], params[1], params[2] == 1);
                } else {
                    System.out.println("Неверный формат. Используйте: roll 3d6 или roll 2d20 --weighted");
                }
            } else {
                System.out.println("Неизвестная команда");
            }
        }
        scanner.close();
    }

    public static void main(String[] args) {
        new DiceRoller().run();
    }
}

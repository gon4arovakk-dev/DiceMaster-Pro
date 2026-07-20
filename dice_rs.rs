// dice_rs.rs — генератор случайных чисел (игральные кости) на Rust

use std::io::{self, Write, BufRead};
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::distributions::Uniform;
use chrono::Local;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct RollEntry {
    timestamp: i64,
    dice: String,
    results: Vec<u32>,
    total: u32,
    weighted: bool,
}

struct DiceRoller {
    history: Vec<RollEntry>,
}

impl DiceRoller {
    fn new() -> Self {
        Self { history: Vec::new() }
    }

    fn roll(&mut self, num_dice: u32, num_faces: u32, weighted: bool) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut results = Vec::new();
        for _ in 0..num_dice {
            let val = if weighted {
                // Смещение: больше вероятность больших значений
                let r: f64 = rng.gen();
                let val = (r * r * num_faces as f64) as u32 + 1;
                if val > num_faces { num_faces } else { val }
            } else {
                rng.gen_range(1..=num_faces)
            };
            results.push(val);
        }
        results
    }

    fn animate_roll(&mut self, num_dice: u32, num_faces: u32, weighted: bool) {
        println!("Бросок{}...", if weighted { " (взвешенный)" } else { "" });
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            print!("\r🎲 ");
            for _ in 0..num_dice {
                print!("{} ", rng.gen_range(1..=num_faces));
            }
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(150));
        }
        let results = self.roll(num_dice, num_faces, weighted);
        let total: u32 = results.iter().sum();
        println!("\r🎲 Результат: {} → Сумма: {}", results.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "), total);
        let entry = RollEntry {
            timestamp: Local::now().timestamp(),
            dice: format!("{}d{}", num_dice, num_faces),
            results,
            total,
            weighted,
        };
        self.history.push(entry);
        self.save_history();
    }

    fn show_stats(&self) {
        if self.history.is_empty() {
            println!("История пуста.");
            return;
        }
        let totals: Vec<u32> = self.history.iter().map(|e| e.total).collect();
        let sum: u32 = totals.iter().sum();
        let avg = sum as f64 / totals.len() as f64;
        let mut sorted = totals.clone();
        sorted.sort();
        let median = sorted[sorted.len() / 2];
        let mut freq = HashMap::new();
        for &v in &totals {
            *freq.entry(v).or_insert(0) += 1;
        }
        let mode = freq.iter().max_by_key(|&(_, count)| count).map(|(&v, _)| v).unwrap_or(0);
        println!("Всего бросков: {}", self.history.len());
        println!("Среднее: {:.2}", avg);
        println!("Медиана: {}", median);
        println!("Мода: {}", mode);
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("История пуста.");
            return;
        }
        for (i, e) in self.history.iter().enumerate() {
            print!("{}. {}", i+1, e.dice);
            if e.weighted { print!(" (взв.)"); }
            println!(" → {} (сумма {})", e.results.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(","), e.total);
        }
    }

    fn clear_history(&mut self) {
        self.history.clear();
        self.save_history();
        println!("История очищена.");
    }

    fn parse_roll(cmd: &str) -> Option<(u32, u32, bool)> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 2 { return None; }
        let dice_str = parts[1];
        if !dice_str.contains('d') { return None; }
        let nums: Vec<&str> = dice_str.split('d').collect();
        if nums.len() != 2 { return None; }
        let num_dice: u32 = nums[0].parse().ok()?;
        let num_faces: u32 = nums[1].parse().ok()?;
        if num_dice == 0 || num_faces == 0 { return None; }
        let weighted = cmd.contains("--weighted");
        Some((num_dice, num_faces, weighted))
    }

    fn save_history(&self) {
        if let Ok(json) = serde_json::to_string(&self.history) {
            let _ = fs::write("dice_history.json", json);
        }
    }

    fn load_history(&mut self) {
        if let Ok(data) = fs::read_to_string("dice_history.json") {
            if let Ok(history) = serde_json::from_str(&data) {
                self.history = history;
            }
        }
    }

    fn run(&mut self) {
        self.load_history();
        println!("🎲 DiceMaster Pro — Rust Edition");
        println!("Команды: roll NDМ [--weighted], history, stats, clear, exit");
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut cmd = String::new();
            if reader.read_line(&mut cmd).is_err() { break; }
            let cmd = cmd.trim();
            if cmd.is_empty() { continue; }
            match cmd {
                "exit" | "quit" => {
                    println!("До свидания!");
                    break;
                }
                "history" => self.show_history(),
                "stats" => self.show_stats(),
                "clear" => self.clear_history(),
                _ => {
                    if cmd.starts_with("roll") {
                        if let Some((num_dice, num_faces, weighted)) = Self::parse_roll(cmd) {
                            self.animate_roll(num_dice, num_faces, weighted);
                        } else {
                            println!("Неверный формат. Используйте: roll 3d6 или roll 2d20 --weighted");
                        }
                    } else {
                        println!("Неизвестная команда");
                    }
                }
            }
        }
    }
}

fn main() {
    let mut roller = DiceRoller::new();
    roller.run();
}

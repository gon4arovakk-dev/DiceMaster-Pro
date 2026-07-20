// dice_go.go — генератор случайных чисел (игральные кости) на Go

package main

import (
	"bufio"
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"strings"
	"time"
)

type RollEntry struct {
	Timestamp int64
	Dice      string
	Results   []int
	Total     int
	Weighted  bool
}

type DiceRoller struct {
	history []RollEntry
}

func NewDiceRoller() *DiceRoller {
	return &DiceRoller{
		history: make([]RollEntry, 0),
	}
}

func (d *DiceRoller) roll(numDice, numFaces int, weighted bool) []int {
	results := make([]int, numDice)
	for i := 0; i < numDice; i++ {
		if weighted {
			// Смещение: вероятность выше для больших чисел
			r := rand.Float64()
			val := int(r*r*float64(numFaces)) + 1
			if val > numFaces {
				val = numFaces
			}
			results[i] = val
		} else {
			results[i] = rand.Intn(numFaces) + 1
		}
	}
	return results
}

func (d *DiceRoller) animateRoll(numDice, numFaces int, weighted bool) {
	fmt.Printf("Бросок%s...\n", map[bool]string{true: " (взвешенный)", false: ""}[weighted])
	for i := 0; i < 5; i++ {
		fmt.Print("\r🎲 ")
		for j := 0; j < numDice; j++ {
			fmt.Printf("%d ", rand.Intn(numFaces)+1)
		}
		time.Sleep(150 * time.Millisecond)
	}
	results := d.roll(numDice, numFaces, weighted)
	total := 0
	for _, v := range results {
		total += v
	}
	fmt.Printf("\r🎲 Результат: %v → Сумма: %d\n", results, total)
	entry := RollEntry{
		Timestamp: time.Now().Unix(),
		Dice:      fmt.Sprintf("%dd%d", numDice, numFaces),
		Results:   results,
		Total:     total,
		Weighted:  weighted,
	}
	d.history = append(d.history, entry)
}

func (d *DiceRoller) showStats() {
	if len(d.history) == 0 {
		fmt.Println("История пуста.")
		return
	}
	totals := make([]int, len(d.history))
	for i, e := range d.history {
		totals[i] = e.Total
	}
	sum := 0
	for _, v := range totals {
		sum += v
	}
	avg := float64(sum) / float64(len(totals))
	// Медиана
	sorted := make([]int, len(totals))
	copy(sorted, totals)
	sortInts(sorted)
	median := sorted[len(sorted)/2]
	// Мода
	freq := make(map[int]int)
	for _, v := range totals {
		freq[v]++
	}
	mode := totals[0]
	maxFreq := 0
	for k, v := range freq {
		if v > maxFreq {
			maxFreq = v
			mode = k
		}
	}
	fmt.Printf("Всего бросков: %d\n", len(d.history))
	fmt.Printf("Среднее: %.2f\n", avg)
	fmt.Printf("Медиана: %d\n", median)
	fmt.Printf("Мода: %d\n", mode)
}

func sortInts(a []int) {
	for i := 0; i < len(a); i++ {
		for j := i + 1; j < len(a); j++ {
			if a[i] > a[j] {
				a[i], a[j] = a[j], a[i]
			}
		}
	}
}

func (d *DiceRoller) showHistory() {
	if len(d.history) == 0 {
		fmt.Println("История пуста.")
		return
	}
	for i, e := range d.history {
		fmt.Printf("%d. %s", i+1, e.Dice)
		if e.Weighted {
			fmt.Print(" (взв.)")
		}
		fmt.Printf(" → %v (сумма %d)\n", e.Results, e.Total)
	}
}

func (d *DiceRoller) clearHistory() {
	d.history = make([]RollEntry, 0)
	fmt.Println("История очищена.")
}

func parseRoll(cmd string) (numDice, numFaces int, weighted bool, ok bool) {
	parts := strings.Fields(cmd)
	if len(parts) < 2 {
		return 0, 0, false, false
	}
	diceStr := parts[1]
	if !strings.Contains(diceStr, "d") {
		return 0, 0, false, false
	}
	nums := strings.Split(diceStr, "d")
	if len(nums) != 2 {
		return 0, 0, false, false
	}
	numDice, err1 := strconv.Atoi(nums[0])
	numFaces, err2 := strconv.Atoi(nums[1])
	if err1 != nil || err2 != nil || numDice <= 0 || numFaces <= 0 {
		return 0, 0, false, false
	}
	weighted = strings.Contains(cmd, "--weighted")
	return numDice, numFaces, weighted, true
}

func main() {
	rand.Seed(time.Now().UnixNano())
	roller := NewDiceRoller()
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Println("🎲 DiceMaster Pro — Go Edition")
	fmt.Println("Команды: roll NDМ [--weighted], history, stats, clear, exit")
	for {
		fmt.Print("> ")
		if !scanner.Scan() {
			break
		}
		cmd := strings.TrimSpace(scanner.Text())
		if cmd == "" {
			continue
		}
		cmdLower := strings.ToLower(cmd)
		switch cmdLower {
		case "exit", "quit":
			fmt.Println("До свидания!")
			return
		case "history":
			roller.showHistory()
		case "stats":
			roller.showStats()
		case "clear":
			roller.clearHistory()
		default:
			if strings.HasPrefix(cmdLower, "roll") {
				numDice, numFaces, weighted, ok := parseRoll(cmd)
				if ok {
					roller.animateRoll(numDice, numFaces, weighted)
				} else {
					fmt.Println("Неверный формат. Используйте: roll 3d6 или roll 2d20 --weighted")
				}
			} else {
				fmt.Println("Неизвестная команда")
			}
		}
	}
}

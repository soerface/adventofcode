from pathlib import Path
from collections import defaultdict

def parse_strategy_guide(input_path: Path) -> dict:
    strategy_guide = defaultdict(str)
    with open(input_path, "r") as f:
        for line in f:
            opponent_choice, player_choice = line.strip().split()
            strategy_guide[opponent_choice] = player_choice
    return strategy_guide

def calculate_score(strategy_guide: dict) -> int:
    score = 0
    for opponent_choice, player_choice in strategy_guide.items():
        if opponent_choice == player_choice:
            # Draw
            score += 3
        elif opponent_choice == "A" and player_choice == "Y":
            # Win
            score += 8
        elif opponent_choice == "B" and player_choice == "X":
            # Win
            score += 8
        elif opponent_choice == "C" and player_choice == "Z":
            # Win
            score += 8
        else:
            # Loss
            score += 0
    return score


def main():
    input_path = Path("input.txt")
    strategy_guide = parse_strategy_guide(input_path)
    total_score = calculate_score(strategy_guide)
    print(total_score)

if __name__ == "__main__":
    main()

from collections import Counter
from typing import List, Dict

PRICE = 800
DISCOUNTS = {1: 0, 2: 40, 3: 80, 4: 160, 5: 200}


def calculate_total(books: List[int]) -> int:
    counts = Counter(books)
    total = 0

    for group_max in range(len(counts), 0, -1):
        group_max_total = calculate_total_with_group_max(dict(counts), group_max)
        total = min(total, group_max_total) if total != 0 else group_max_total

    return total


def calculate_total_with_group_max(counts: Dict[int, int], group_max: int) -> int:
    total = 0

    while True:
        group = 0

        for book in counts:
            if counts[book]:
                counts[book] -= 1
                group += 1

            if group >= group_max:
                break

        if not group:
            return total

        total += (PRICE - DISCOUNTS[group]) * group

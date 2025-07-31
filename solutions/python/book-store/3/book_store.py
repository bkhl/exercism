from collections import Counter
from typing import List, Dict

PRICE = 800
DISCOUNTS = {1: 0, 2: 40, 3: 80, 4: 160, 5: 200}


def calculate_total(books: List[int]) -> int:
    counts = Counter(books)
    books_by_count = [
        book for book, _ in sorted(counts.items(), key=lambda item: -item[1])
    ]
    total = 0

    for group_max in range(len(counts), 0, -1):
        group_max_total = calculate_total_with_group_max(
            dict(counts), books_by_count, group_max
        )
        total = min(total, group_max_total) if total != 0 else group_max_total

    return total


def calculate_total_with_group_max(
    counts: Dict[int, int], books_by_count: int, group_max: int
) -> int:
    total = 0

    while True:
        group_size = 0

        for book in books_by_count:
            if counts[book]:
                counts[book] -= 1
                group_size += 1

            if group_size == group_max:
                break

        if not group_size:
            return total

        total += (PRICE - DISCOUNTS[group_size]) * group_size

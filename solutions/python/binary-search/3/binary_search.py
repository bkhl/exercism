def binary_search(list_of_numbers, number):
    lower = 0
    upper = len(list_of_numbers) - 1

    while lower <= upper:
        mid = (lower + upper) // 2
        value = list_of_numbers[mid]

        if value < number:
            lower = mid + 1
        elif value > number:
            upper = mid - 1
        else:
            return mid

    raise ValueError("number not in list")

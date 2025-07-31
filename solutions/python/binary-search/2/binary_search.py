def binary_search(list_of_numbers, number):
    if len(list_of_numbers) == 0:
        raise ValueError("number not in list")

    lower = 0
    upper = len(list_of_numbers) - 1
    mid = upper // 2

    while True:
        value = list_of_numbers[mid]

        if value == number:
            return mid

        if lower == upper:
            raise ValueError("number not in list")

        if value < number:
            lower = mid + 1
        else:
            upper = mid - 1

        mid = (upper + lower) // 2

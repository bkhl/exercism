EQUAL = "EQUAL"
SUBLIST = "SUBLIST"
SUPERLIST = "SUPERLIST"
UNEQUAL = "UNEQUAL"


def is_sublist(first_list, second_list):
    for i in range(0, len(second_list) - len(first_list) + 1):
        if first_list == second_list[i : i + len(first_list)]:
            return True

    return False


def check_lists(first_list, second_list):
    if len(first_list) < len(second_list):
        if is_sublist(first_list, second_list):
            return SUBLIST
        else:
            return UNEQUAL
    elif len(first_list) > len(second_list):
        if is_sublist(second_list, first_list):
            return SUPERLIST
        else:
            return UNEQUAL
    else:
        if first_list == second_list:
            return EQUAL
        else:
            return UNEQUAL

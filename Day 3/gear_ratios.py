##
# gear_ratios.py
# Date: 05/12/2023
# Author: Ryan Gordon
# Response for advent of code day 3 2023

from functools import reduce

def find_adjacent_part_numbers(row: int, col: int) -> list:
    """
    Finds all part numbers adjacent to a given entry and returns as a list.

    row -- the row of the entry.
    col -- the column of the entry.
    """
    part_numbers = []
    indices_covered = []
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i == 0 and j == 0:
                continue

            if [i, j] in indices_covered:
                continue

            try:
                int(all_lines[row + i][col + j])

                start_index = col + j
                while True:
                    try:
                        int(all_lines[row + i][start_index])
                    except (ValueError, IndexError):
                        start_index += 1
                        break

                    start_index -= 1

                number, offset = "", start_index - (col + j)
                
                while True:
                    character = all_lines[row + i][start_index]
                    try:
                        int(character)
                        number += character
                        indices_covered.append([i, j + offset])
                    except (ValueError, IndexError):
                        break

                    start_index += 1
                    offset += 1

                part_numbers.append(int(number))
            except ValueError:
                pass
    return part_numbers

SYMBOLS = tuple("@#$%&*-=+/")

global all_lines
all_lines = []
with open("engine.txt", "r") as reader:
    for line in reader.readlines():
        all_lines.append(list(line))

if __name__ == "__main__":
    part_numbers_sum = 0
    gear_ratios_sum = 0
    section = ""
    for i, line in enumerate(all_lines):
        for j, char in enumerate(line):
            character = all_lines[i][j]

            # Part 2
            if character == "*":
                # Search for the presence of two adjacent part numbers.
                part_numbers = find_adjacent_part_numbers(i, j)
                if len(part_numbers) == 2:
                    gear_ratios_sum += reduce(lambda x, y: x*y, part_numbers)

            # Part 1            
            try:
                int(character)
                section += character

                if j == len(line) - 1:
                    raise ValueError()
            except ValueError:
                if len(section) == 0:
                    continue

                # Add number if applicable.
                symbol_present = False
                
                if character in SYMBOLS:
                    symbol_present = True

                # Search to left and right of number.
                try:
                    if all_lines[i][j - len(section) - 1] in SYMBOLS:
                        symbol_present = True
                except IndexError:
                    pass

                # Search above and below number.
                for k in range((len(section) + 1) * -1, 1):
                    try:
                        if all_lines[i - 1][j + k] in SYMBOLS:
                            symbol_present = True
                    except IndexError:
                        pass

                    try:
                        if all_lines[i + 1][j + k] in SYMBOLS:
                            symbol_present = True
                    except IndexError:
                        pass

                    if symbol_present:
                        break

                if symbol_present:
                    part_numbers_sum += int(section)

                section = ""
    print(f"Part 1: {part_numbers_sum}")
    print(f"Part 2: {gear_ratios_sum}")

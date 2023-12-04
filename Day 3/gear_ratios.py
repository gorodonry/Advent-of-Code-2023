##
# gear_ratios.py
# Date: 05/12/2023
# Author: Ryan Gordon
# Response for advent of code day 3 2023

NON_SYMBOLS = tuple("@#$%&*-=+/")

all_lines = [None for i in range(sum(1 for line in open("engine.txt", "r")))]
with open("engine.txt", "r") as reader:
    for i, line in enumerate(reader.readlines()):
        all_lines[i] = list(line)

if __name__ == "__main__":
    total = 0
    section = ""
    for i, line in enumerate(all_lines):
        for j, char in enumerate(line):
            character = all_lines[i][j]
            
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
                
                if character in NON_SYMBOLS:
                    symbol_present = True

                # Search to left and right of number.
                try:
                    if all_lines[i][j - len(section) - 1] in NON_SYMBOLS:
                        symbol_present = True
                except IndexError:
                    pass

                # Search above and below number.
                for k in range((len(section) + 1) * -1, 1):
                    try:
                        if all_lines[i - 1][j + k] in NON_SYMBOLS:
                            symbol_present = True
                    except IndexError:
                        pass

                    try:
                        if all_lines[i + 1][j + k] in NON_SYMBOLS:
                            symbol_present = True
                    except IndexError:
                        pass

                    if symbol_present:
                        break

                if symbol_present:
                    total += int(section)

                section = ""
    print(total)

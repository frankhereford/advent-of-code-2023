# Define the file path
input = './input/day_01_test.txt'

# Using a context manager to open and read the file
with open(input, 'r') as file:
    file_contents = file.read()

# Splitting the file contents into lines
lines = file_contents.splitlines()

found_digits = []
# Iterating over each line, splitting it into characters, and tracking digits
for line in lines:
    print(f"Line: {line}")
    chars = list(line)
    first_digit_index = None
    last_digit_index = None

    for i, char in enumerate(chars):
        if char.isdigit():
            if first_digit_index is None:
                first_digit_index = i
            last_digit_index = i

    print(f"Line: {line}")
    print(f"First digit at index: {first_digit_index}, Last digit at index: {last_digit_index}")
    str_number = line[first_digit_index] + line[last_digit_index]
    number = int(str_number)
    print(f"Number: {number}")
    found_digits.append(number)

print("Part 1: ", sum(found_digits))

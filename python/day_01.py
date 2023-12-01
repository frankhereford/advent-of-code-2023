# ! README

# This solution used GPT4 as a python documentation reference.

# I'm sticking this right up here because I want to show what a powerful documentation / reference tool GPT 4 is.
# I'm very intentionally not using it to help me solve the creation of an algorithm to solve the problem.
# I'm using it as an exceptionally reference / documentation when writing Python.

# If one were to let GPT4 solve it, they might get through today (maybe?), but it's not going to pan out long term.
# You have to understand how to herd it. You have to understand how to use it as a tool.

# I know it sounds dodgy, so I'm going to include every single GPT 4 interaction log related to AOC 
# in the comments, as a link. I'm excited to show you the ways that I've used it. I really hope anyone 
# reading this goes on to read some of the interactions with GPT-4 and thinks "Wow, that's really cool. 
# I want to try to learn how to use that."

# It's a really powerful tool.

# Here's what I used GPT 4 for when writing this:

# * Reading a file into a variable in Python.
# * Splitting file contents into lines and iterating over them.
# * Combining file reading and line iteration in Python.
# * Summing the elements of an array in Python.
# * Removing the first 4 elements from an array in Python.
# * Returning the index of a unique value in a list.
# * Splicing a value into an array in Python.
# * Understanding the slicing syntax in Python, specifically `line[search_index:number_length]`.
# * Checking if a character at a certain index in a string is one of several specified characters.
# * Converting the ASCII number of a textual digit to its actual integer value.

# See: https://chat.openai.com/share/c4f3e57d-e7d7-4be8-9e4e-32bffc66eed2

#input = './input/day_01_part_1_test.txt'
input = './input/day_01_real.txt'

with open(input, 'r') as file:
    file_contents = file.read()

lines = file_contents.splitlines()

found_digits = []
for line in lines:
    chars = list(line)
    first_digit_index = None
    last_digit_index = None

    for i, char in enumerate(chars):
        if char.isdigit():
            if first_digit_index is None:
                first_digit_index = i
            last_digit_index = i

    str_number = line[first_digit_index] + line[last_digit_index]
    number = int(str_number)
    found_digits.append(number)

print("Part 1: ", sum(found_digits))

#input = './input/day_01_part_2_test.txt'

with open(input, 'r') as file:
    file_contents = file.read()

lines = file_contents.splitlines()

numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

total = 0
for line in lines:
    search_index = 0
    first_digit_index = None
    first_digit_value = None
    last_digit_index = None
    last_digit_value = None
    while search_index <= len(line):
        for number in numbers:
            number_length = len(number)
            if number == line[search_index:number_length + search_index]:
                if first_digit_index is None:
                    first_digit_index = search_index
                    first_digit_value = numbers.index(number) + 1
                last_digit_index = search_index
                last_digit_value = numbers.index(number) + 1
        if search_index < len(line):
            try:
                if int(line[search_index]) in (1,2,3,4,5,6,7,8,9,0):
                    if first_digit_index is None:
                        first_digit_index = search_index
                        first_digit_value = line[search_index]
                    last_digit_index = search_index
                    last_digit_value = line[search_index]
            except:
                pass
        search_index += 1
    combined_digits = f"{first_digit_value}{last_digit_value}"
    combined_value = int(combined_digits)
    total += combined_value
print("Part 2: ", total)

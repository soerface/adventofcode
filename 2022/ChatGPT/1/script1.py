# Open the input file and read its contents
with open('input.txt') as file:
  input_str = file.read()

# Split the input by blank lines
food_lists = [list(map(int, x.strip().split('\n'))) for x in input_str.strip().split('\n\n')]

# Calculate the total Calories carried by each Elf
totals = [sum(food_list) for food_list in food_lists]

# Find the maximum Calories carried by any Elf
max_calories = max(totals)

# Print the result
print(max_calories)
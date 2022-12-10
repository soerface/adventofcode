# Open the input file and read its contents
with open('input.txt') as file:
  input_str = file.read()

# Split the input by blank lines
food_lists = [list(map(int, x.strip().split('\n'))) for x in input_str.strip().split('\n\n')]

# Calculate the total Calories carried by each Elf
totals = [sum(food_list) for food_list in food_lists]

# Sort the list of total Calories in descending order
totals = sorted(totals, reverse=True)

# Take the top three Elves and find the total Calories they are carrying
top_three_total = sum(totals[:3])

# Print the result
print(top_three_total)
# Read the input from the file.
with open('input.txt') as f:
    rucksacks = f.readlines()

# Split the list of items in each rucksack into two compartments.
compartments = [list(items)[:len(items)//2] for items in rucksacks]

# Create a set of items in each compartment.
sets = [set(compartment) for compartment in compartments]

# Take the intersection of the sets to find the items that appear in both compartments.
common_items = set.intersection(*sets)

# Sum the priorities of the common items.
priority_sum = 0
for item in common_items:
    priority = 1 + ord(item) - ord('a')
    priority_sum += priority

# Print the sum of the priorities of the common items.
print(priority_sum)

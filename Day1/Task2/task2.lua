local file = io.open("lists.txt", "r")
local list1 = {} --list of location id 1
local list2 = {} --list of location id 2

if not file then
    print("Error: Cannot open file")
    return
end

for line in file:lines() do 
    local a, b = line:match("(%d+)%s+(%d+)") 
    table.insert(list1, a)
    table.insert(list2, b)
end

file:close()

local frequency_table = {}

for index, value in ipairs(list2) do
    if frequency_table[value] then
        frequency_table[value] = frequency_table[value] + 1
    else
        frequency_table[value] = 1
    end
end

local similarity_score_total = 0

for index, value in ipairs(list1) do
    if frequency_table[value] then
        local similarity_score = value * frequency_table[value]
        similarity_score_total = similarity_score_total + similarity_score
    end
end

print(similarity_score_total)
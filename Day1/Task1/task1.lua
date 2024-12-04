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

table.sort(list1)
table.sort(list2)

local total_distanse = 0

for i = 1, #list1 do
    local distanse = math.abs(list1[i] - list2[i])
    total_distanse = total_distanse + distanse
end

print(total_distanse)




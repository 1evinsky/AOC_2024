local function read_file(path)
  local file = io.open(path, "r")
  if not file then
      print("Error: Cannot open file")
      return nil
  end
  local str = file:read("*all")
  file:close()
  return str
end

local function parser(text)
  local resault = {}

  for num1, num2 in text:gmatch("mul%((%d+),(%d+)%)") do
    table.insert(resault, {num1 = tonumber(num1), num2 = tonumber(num2)})
  end
  
  return resault
end

local function calculate_total(parse_resault)
  local total = 0
  for i,v in pairs(parse_resault) do
    local mul = v.num1 * v.num2
    total = total + mul
  end
  return total
end

local function main()
    local str = read_file("./Day3/Task1/input.txt")
    local parse_resault = parser(str)
    local total = calculate_total(parse_resault)
    print(total)
end

main()

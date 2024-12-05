import os
print(os.getcwd())

input = open("./Day2/Task1/input.txt", "r")

reports = []

for line in input:
    reports.append(line.split())

safe = 0
    
for report in reports:
    prev_increment = False
    is_Setup = False
    for i in range(0, len(report)):

        if(i + 1 > len(report) - 1): # end of the list, all good
            safe += 1
            break

        current = int(report[i])
        next = int(report[i+1])

        if(current == next): # same number, skip
            break

        if(current < next): 
            current_increment = True
            if((next - current <= 3) == False): # not safe because the difference is more than 3
                break 
        else:
            current_increment = False
            if((current - next <= 3) == False): # not safe because the difference is more than 3
                break
        
        if(is_Setup == False):
            prev_increment = current_increment
            is_Setup = True
            continue
                    
        if(prev_increment != current_increment): # not safe because the increment is not the same as the previous increment
            break
        else:
            prev_increment = current_increment
            

print("unsafe=", len(reports)-safe, "safe=", safe, "total=", len(reports))
            
        
    
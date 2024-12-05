import os

def read_input(file_path):
    input = open(file_path, "r")
    reports = []
    for line in input:
        reports.append(line.split())
    return reports
        
def analize_reports(report):
    prev_increment = False
    is_Setup = False
    for i in range(0, len(report)):

        if(i + 1 > len(report) - 1): # end of the list, all good
            return True, -1

        current = int(report[i])
        next = int(report[i+1])

        if(current == next): # same number, skip
            return False, i

        if((abs(current - next) <= 3) == False): # not safe because the difference is more than 3
            return False, i 
            
        if(current < next): 
            current_increment = True
        else:
            current_increment = False
        
        if(is_Setup == False):
            prev_increment = current_increment
            is_Setup = True
            continue
                    
        if(prev_increment != current_increment): # not safe because the increment is not the same as the previous increment
            if (i == 1):
                return False, i - 1 # first element is the problem, we skip it on setup 
            else:
                return False, i


def problem_dumper(report, problem_index):
    tmp_report = report.copy()
    del tmp_report[problem_index]
    return tmp_report

if __name__ == "__main__":
    print(os.getcwd())

    reports = read_input("./Day2/Task2/input.txt")

    safe = 0

    for report in reports:
        is_safe, problem_index = analize_reports(report)
        
        if(is_safe == True):
            safe += 1
            continue
        
        fix_report1 = problem_dumper(report, problem_index)
        
        is_safe, index = analize_reports(fix_report1)
        
        if(is_safe == True):
            safe += 1
            continue

        fix_report2 = problem_dumper(report, problem_index + 1)

        is_safe, wtf_index = analize_reports(fix_report2)

        if(is_safe == True):
            safe += 1

    
    print("unsafe=", len(reports)-safe, "safe=", safe, "total=", len(reports))


           

iterations1 = 40
iterations2 = 50
start = '1113222113'

def look_and_say(number):
    digits = []
    digit_count = 1
    last_digit = number[0]
    for digit in number[1:]:
        if digit == last_digit:
            digit_count += 1
        else:
            digits.append([last_digit, digit_count])
            digit_count = 1
        last_digit = digit
    digits.append([last_digit, digit_count])
    return ''.join([str(n) + d for (d, n) in digits])


result = start

for _ in range(iterations1):
    result = look_and_say(result)

print(len(result))


for _ in range(iterations2 - iterations1):
    result = look_and_say(result)

print(len(result))

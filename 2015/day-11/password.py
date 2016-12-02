import re

double = re.compile(r'(([a-z])\2)')
exclude = ['i', 'l', 'o']

def is_good(password):
    if any([letter in password for letter in exclude]):
        return False
    if len(double.findall(password)) < 2:
        return False
    has_triple_sequence = False
    for i in range(len(password) - 2):
        first, second, third = map(ord, password[i:i+3])
        if second - first == 1:
            if third - second == 1:
                return True
    return False


def increment_password(password):
    new_password = password
    good = False
    while not good:
        letters = list(reversed(list(new_password)))
        carry = False
        for i, letter in enumerate(letters):
            if letter < 'z':
                carry = False
                letters[i] = chr(ord(letter) + 1)
            else:
                carry = True
                letters[i] = 'a'
            if not carry:
                break
        new_password = ''.join(reversed(letters)) 
        if is_good(new_password):
            good = True
    return new_password

pw = 'vzbxkghb'

pw = increment_password(pw)
print(pw)

pw = increment_password(pw)
print(pw)

import re


vowel = re.compile(r'[aeiou]')
double = re.compile(r'([a-z])\1')
naughty_pairs = ['ab', 'cd', 'pq', 'xy']


def is_nice(name):
    has_three_vowels = len(vowel.findall(name)) >= 3
    has_double = len(double.findall(name)) >= 1
    naughty = False
    for pair in naughty_pairs:
        if pair in name:
            naughty = True
            break
    return all([has_three_vowels, has_double, not naughty])


two_pairs = re.compile(r'([a-z][a-z]).*\1')
letter_sandwich = re.compile(r'([a-z]).\1')


def is_nice2(name):
    has_two_pairs = len(two_pairs.findall(name)) >= 1
    has_sandwich = len(letter_sandwich.findall(name)) >= 1
    return has_two_pairs and has_sandwich


n_nice = 0
n_nice2 = 0
total = 0
with open('names.txt', 'r') as f:
    for line in f.readlines():
        total += 1
        name = line.strip().lower()       
        if is_nice(name):
            n_nice += 1
        if is_nice2(name):
            n_nice2 += 1

print('{0} of {1} are nice'.format(n_nice, total))
print('{0} of {1} are nice2'.format(n_nice2, total))

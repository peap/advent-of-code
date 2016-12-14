from pprint import pprint
import re
import sys

wires = {}
signals = {}

and_re = re.compile('([0-9a-z]+) AND ([0-9a-z]+)')
or_re = re.compile('([0-9a-z]+) OR ([0-9a-z]+)')
lshift_re = re.compile('([a-z]+) LSHIFT ([0-9]+)')
rshift_re = re.compile('([a-z]+) RSHIFT ([0-9]+)')
not_re = re.compile('NOT ([a-z]+)')

def as_int(instr):
    try:
        return int(instr)
    except ValueError:
        return None
    

def get_signal(wire, all_wires, all_signals):
    if wire in all_signals:
        return all_signals[wire]
    instr = all_wires[wire]

    signal = as_int(instr)
    if signal is not None:
        all_signals[wire] = signal
        return signal

    # NOT
    instr_re = not_re.match(instr)
    if instr_re:
        (a,) = instr_re.groups()
        return ~ get_signal(a, all_wires, all_signals) % 2**16
    # AND
    instr_re = and_re.match(instr)
    if instr_re:
        (a, b) = instr_re.groups()
        left = as_int(a)
        if left is None:
            left = get_signal(a, all_wires, all_signals)
            all_signals[a] = left
        right = as_int(b)
        if right is None:
            right = get_signal(b, all_wires, all_signals)
            all_signals[b] = right
        return left & right
    # OR
    instr_re = or_re.match(instr)
    if instr_re:
        (a, b) = instr_re.groups()
        left = as_int(a)
        if left is None:
            left = get_signal(a, all_wires, all_signals)
            all_signals[a] = left
        right = as_int(b)
        if right is None:
            right = get_signal(b, all_wires, all_signals)
            all_signals[b] = right
        return left | right
    # LSHIFT
    instr_re = lshift_re.match(instr)
    if instr_re:
        (a, num) = instr_re.groups()
        left = get_signal(a, all_wires, all_signals)
        all_signals[a] = left
        return left << int(num)
    # RSHIFT
    instr_re = rshift_re.match(instr)
    if instr_re:
        (a, num) = instr_re.groups()
        left = get_signal(a, all_wires, all_signals)
        all_signals[a] = left
        return left >> int(num)

    other_wire = instr.strip()
    if len(other_wire.split()) == 1:
        return get_signal(other_wire, all_wires, all_signals)

    raise Exception('unhandled input: ' + instr)


with open('input.txt', 'r') as f:
    lines = f.readlines()

print('reading instructions')
for line in lines:
    instruction = line.strip()
    left, right = instruction.split(' -> ')
    wires[right] = left

print('computing signals\n')
for wire in wires:
    if wire not in signals:
        signals[wire] = get_signal(wire, wires, signals)

print('a -> {0}'.format(signals['a']))

signals2 = {}
wires2 = wires.copy()
wires2['b'] = str(signals['a'])

print('computing signals again: ')
for wire in wires2:
    if wire not in signals2:
        signals2[wire] = get_signal(wire, wires2, signals2)

print('a -> {0}'.format(signals2['a']))

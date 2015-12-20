from md5 import md5

prefix = 'ckczppom'

i = 0
found5 = False
found6 = False
while not found5 or not found6:
    m = md5(prefix + str(i))
    if m.hexdigest().startswith('000000'):
        print('6 zeros: {0}'.format(i))
        found6 = True
    if m.hexdigest().startswith('00000'):
        print('5 zeros: {0}'.format(i))
        found5 = True
    i += 1

import hashlib

door_id = 'ojvtpuvg'
index = 0

password_1 = ''
password_2 = ['', '', '', '', '', '', '', '']

while len(password_1) < 8 or len(''.join(password_2)) < 8:
    h = hashlib.md5((door_id + str(index)).encode('utf8')).hexdigest()
    index += 1

    if h[:5] == '00000':
        if len(password_1) < 8:
            password_1 += h[5]

        try:
            position = int(h[5])
        except:
            continue

        if position < 8 and password_2[position] == '':
            password_2[position] = h[6]



print('#1:', password_1)
print('#2:', ''.join(password_2))

# 2024 Paweł Rybak

import sys


def prepare(key_len):
    with open("orig.txt", "r") as f:
        text = f.read().replace("\n", " ")

    text = ''.join(c.lower() for c in text if c.isascii()
                   and (c.isalpha() or c == ' '))

    with open("plain.txt", "w") as f:
        for i in range(0, len(text), key_len):
            chunk = text[i:i+key_len]
            f.write(f"{chunk:<{key_len}}\n")


def text_cleanup(input_str):
    ''.join(c.lower()
            for c in input_str if c.isascii() and (c.isalpha() or c == ' '))


def encrypt():
    with open("key.txt", "r") as file:
        key = file.read()

    with open("plain.txt", "r") as file:
        lines = file.read().splitlines()

    encrypted = []
    for line in lines:
        encrypted_line = ""
        for i in range(len(line)):
            xor_value = ord(line[i]) ^ ord(key[i])
            encrypted_line += chr(xor_value)
        encrypted.append(encrypted_line)

    output = ''.join(encrypted)

    with open("crypto.txt", "w") as file:
        file.write(output)


def cryptanalysis(key_len):
    with open("crypto.txt", "rb") as file:
        encrypted_text = file.read()

    encrypted_lines = []
    final_keys = []

    for i in range(0, len(encrypted_text), key_len):
        chunk = encrypted_text[i:i + key_len]
        line = [byte for byte in chunk]
        encrypted_lines.append(line)

    for i in range(key_len):
        key_not_space = False
        key_found = -1

        for j in range(len(encrypted_lines)):
            if encrypted_lines[j][i] < 32 and encrypted_lines[j][i] > 0:
                key_not_space = True
                break

        if key_not_space:
            for j in range(len(encrypted_lines)):
                if encrypted_lines[j][i] >= 64:
                    key_found = encrypted_lines[j][i] ^ 32
                    break
        else:
            for j in range(1, len(encrypted_lines)):
                if encrypted_lines[j][i] != encrypted_lines[j - 1][i]:
                    key_found = 32
                    break

        final_keys.append(key_found)

    for i in range(key_len):
        for j in range(len(encrypted_lines)):
            if final_keys[i] == -1:
                encrypted_lines[j][i] = 95
            else:
                encrypted_lines[j][i] = encrypted_lines[j][i] ^ final_keys[i]

    char_lines = [''.join(chr(num) for num in line)
                  for line in encrypted_lines]

    file_content = "\n".join(char_lines)
    with open("decrypt.txt", "w") as file:
        file.write(file_content)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python xor.py -p|-e|-k")
        sys.exit(1)

    with open("key.txt", "r") as f:
        key_len = len(f.read())

    option = sys.argv[1]
    if option == '-p':
        prepare(key_len)
    elif option == '-e':
        encrypt()
    elif option == '-k':
        cryptanalysis(key_len)
    else:
        print("Invalid option. Use -p, -e or -k.")

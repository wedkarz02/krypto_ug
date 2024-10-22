# 2024 Paweł Rybak

import sys


def prepare(key_len):
    with open("orig.txt", "r") as file:
        text = file.read().replace("\n", " ")

    text = "".join(c.lower() for c in text if c.isascii() and (c.isalpha() or c == ' '))
    with open("plain.txt", "w") as file:
        for i in range(0, len(text), key_len):
            chunk = text[i:i+key_len]
            file.write(f"{chunk:<{key_len}}\n")


def encrypt():
    with open("key.txt", "r") as file:
        key = file.read()

    with open("plain.txt", "r") as file:
        lines = file.read().splitlines()

    out_lines = []
    for line in lines:
        encoded_line = ""
        for i in range(len(line)):
            encoded_line += chr(ord(line[i]) ^ ord(key[i]))
        out_lines.append(encoded_line)

    with open("crypto.txt", "w") as file:
        file.write("".join(out_lines))


def cryptanalysis(key_len):
    with open("crypto.txt", "rb") as file:
        ciphertext = file.read()

    cipher_lines = []
    for i in range(0, len(ciphertext), key_len):
        cipher_lines.append([b for b in ciphertext[i:i + key_len]])

    found_chars = []
    for i in range(key_len):
        is_space = True
        for j in range(len(cipher_lines)):
            if cipher_lines[j][i] < 32 and cipher_lines[j][i] > 0:
                is_space = False
                break

        key = -1
        if not is_space:
            for j in range(len(cipher_lines)):
                if cipher_lines[j][i] >= 64:
                    key = cipher_lines[j][i] ^ 32
                    break
        else:
            for j in range(1, len(cipher_lines)):
                if cipher_lines[j][i] != cipher_lines[j - 1][i]:
                    key = 32
                    break

        found_chars.append(key)

    for i in range(key_len):
        for j in range(len(cipher_lines)):
            if found_chars[i] == -1:
                cipher_lines[j][i] = ord("_")
            else:
                cipher_lines[j][i] = cipher_lines[j][i] ^ found_chars[i]

    with open("decrypt.txt", "w") as file:
        file.write("\n".join(["".join(chr(num) for num in line) for line in cipher_lines]))


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 xor.py <-p|-e|-k>")
        sys.exit(1)

    try:
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
    except Exception as err:
        print(err)

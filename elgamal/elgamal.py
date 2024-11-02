# 2024 Paweł Rybak

import sys
import random
from math import gcd


def mod_inverse(a, n):
    t, new_t = 0, 1
    r, new_r = n, a
    while new_r != 0:
        quotient = r // new_r
        t, new_t = new_t, t - quotient * new_t
        r, new_r = new_r, r - quotient * new_r

    if r > 1:
        return None

    if t < 0:
        t += n

    return t


def load_pg(filename='elgamal.txt'):
    with open(filename, 'r') as f:
        p = int(f.readline().strip())
        g = int(f.readline().strip())
    return p, g


def save_key(filename, p, g, value):
    with open(filename, 'w') as f:
        f.write(f"{p}\n{g}\n{value}\n")


def generate_keys():
    p, g = load_pg()
    b = random.randint(1, p - 2)
    beta = pow(g, b, p)
    save_key('private.txt', p, g, b)
    save_key('public.txt', p, g, beta)


def text_to_number(text):
    return int.from_bytes(text.encode('utf-8'), 'big')


def number_to_text(number):
    return number.to_bytes((number.bit_length() + 7) // 8, 'big').decode('utf-8')


def encrypt():
    p, g, beta = load_key('public.txt')
    with open('plain.txt', 'r') as f:
        text = f.read().strip()

    m = text_to_number(text)
    if not (0 < m < p):
        print("Error: 0 < m < p not satisfied")
        return

    k = random.randint(1, p - 2)
    c1 = pow(g, k, p)
    c2 = (m * pow(beta, k, p)) % p
    with open('crypto.txt', 'w') as f:
        f.write(f"{c1}\n{c2}\n")


def decrypt():
    p, g, b = load_key('private.txt')
    with open('crypto.txt', 'r') as f:
        c1 = int(f.readline().strip())
        c2 = int(f.readline().strip())

    beta_k = pow(c1, b, p)
    beta_k_inv = mod_inverse(beta_k, p)
    m = (c2 * beta_k_inv) % p

    try:
        text = number_to_text(m)
    except ValueError:
        print("Error: Failed to convert number to text")
        return

    with open('decrypt.txt', 'w') as f:
        f.write(f"{text}\n")


def sign():
    p, g, b = load_key('private.txt')
    with open('message.txt', 'r') as f:
        text = f.read().strip()

    m = text_to_number(text)
    if not (0 < m < p):
        print("Error: 0 < m < p not satisfied")
        return

    p_minus_1 = p - 1
    while True:
        k = random.randint(1, p_minus_1 - 1)
        if gcd(k, p_minus_1) == 1:
            break

    r = pow(g, k, p)
    k_inv = mod_inverse(k, p_minus_1)
    x = ((m - b * r) * k_inv) % p_minus_1
    with open('signature.txt', 'w') as f:
        f.write(f"{r}\n{x}\n")


def verify():
    p, g, beta = load_key('public.txt')
    with open('message.txt', 'r') as f:
        text = f.read().strip()

    m = text_to_number(text)
    with open('signature.txt', 'r') as f:
        r = int(f.readline().strip())
        x = int(f.readline().strip())

    left = pow(g, m, p)
    right = (pow(r, x, p) * pow(beta, r, p)) % p
    with open('verify.txt', 'w') as f:
        if left == right:
            print("T")
            f.write("T\n")
        else:
            print("N")
            f.write("N\n")


def load_key(filename):
    with open(filename, 'r') as f:
        p = int(f.readline().strip())
        g = int(f.readline().strip())
        value = int(f.readline().strip())
    return p, g, value


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python elgamal.py -k|-e|-d|-s|-v")
        sys.exit()

    option = sys.argv[1]
    if option == '-k':
        generate_keys()
    elif option == '-e':
        encrypt()
    elif option == '-d':
        decrypt()
    elif option == '-s':
        sign()
    elif option == '-v':
        verify()
    else:
        print("Usage: python elgamal.py -k|-e|-d|-s|-v")

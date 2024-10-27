# 2024 Paweł Rybak

def count_diff(hash1, hash2):
    bin1 = bin(int(hash1, 16))[2:].zfill(len(hash1) * 4)
    bin2 = bin(int(hash2, 16))[2:].zfill(len(hash2) * 4)
    return sum(b1 != b2 for b1, b2 in zip(bin1, bin2)), len(bin1)


if __name__ == "__main__":
    with open("hash.txt", "r") as f:
        lines = f.readlines()

    hash_pairs = [(lines[i].strip().split()[0], lines[i+1].strip().split()[0])
                  for i in range(0, len(lines), 2)]
    hash_functions = {
        0: "md5",
        1: "sha1",
        2: "sha224",
        3: "sha256",
        4: "sha384",
        5: "sha512",
        6: "b2",
    }

    with open("diff.txt", "w") as diff_file:
        for i, (hash1, hash2) in enumerate(hash_pairs):
            diff_bits, total_bits = count_diff(hash1, hash2)
            percent_diff = (diff_bits / total_bits) * 100
            diff_file.write(f"{hash_functions[i]}:\n{hash1}\n{hash2}\n")
            diff_file.write(
                f"Liczba różniących się bitów: {diff_bits} z {total_bits}, procentowo: {percent_diff:.2f}%\n\n")

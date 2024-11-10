# 2024 Paweł Rybak

import sys


def html_cleanup(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        html_content = file.read()

    lines = html_content.splitlines()
    no_indent_html = "\n".join(line.strip() for line in lines if line.strip())
    with open(file_path, 'w', encoding='utf-8') as file:
        file.write(no_indent_html)


def hex_to_bin(hex_string):
    return ''.join(f"{int(char, 16):04b}" for char in hex_string)


def read_message(file_path):
    with open(file_path, 'r') as file:
        hex_data = file.read().strip()
    return hex_to_bin(hex_data)


def hide_message_in_spaces(lines, bits):
    if len(bits) > len(lines):
        raise ValueError("Not enough lines in destination data")
    return [
        line.rstrip() + (" " if bit == "1" else "")
        for line, bit in zip(lines, bits)
    ] + lines[len(bits):]


def extract_message_from_spaces(lines):
    bits = ["1" if line.endswith(" ") else "0" for line in lines]
    return ''.join(bits)


def hide_message_in_double_spaces(content, bits):
    modified_content = []
    bit_index = 0

    i = 0
    while i < len(content):
        if content[i] == ' ' and (i + 1 >= len(content) or content[i + 1] != ' '):
            if bit_index < len(bits):
                modified_content.append(' ' * (1 + int(bits[bit_index])))
                bit_index += 1
            else:
                modified_content.append(' ')
            i += 1
        else:
            modified_content.append(content[i])
            i += 1

    if bit_index < len(bits):
        raise ValueError("Not enough spaces in destination data")

    return ''.join(modified_content)


def extract_message_from_double_spaces(content):
    bits = []
    i = 0
    while i < len(content):
        if content[i] == ' ' and (i + 1 >= len(content) or content[i + 1] != ' '):
            bits.append('0')
            i += 1
        elif content[i:i+2] == '  ':
            bits.append('1')
            i += 2
        else:
            i += 1

    return ''.join(bits)


def hide_message_in_typo_attributes(content, bits):
    modified_content = []
    bit_index = 0
    inside_p_tag = False

    i = 0
    while i < len(content):
        if content[i:i+2] == '<p':
            inside_p_tag = True
            modified_content.append('<p')
            i += 2
        elif inside_p_tag and content[i] == '>':
            if bit_index < len(bits):
                if bits[bit_index] == '1':
                    modified_content.append(' style="lineheight: 100%"')
                else:
                    modified_content.append(' style="margin-botom: 0cm"')
                bit_index += 1
            modified_content.append('>')
            inside_p_tag = False
            i += 1
        else:
            modified_content.append(content[i])
            i += 1

    if bit_index < len(bits):
        raise ValueError("Not enough <p> tags to hide the message")

    return ''.join(modified_content)


def extract_message_from_typo_attributes(content):
    bits = []
    inside_p_tag = False

    i = 0
    while i < len(content):
        if content[i:i+2] == '<p':
            inside_p_tag = True
            i += 2
        elif inside_p_tag and content[i] == '>':
            inside_p_tag = False
            i += 1
        elif inside_p_tag:
            if content[i:i+18] == ' style="lineheight':
                bits.append('1')
                i += 18
            elif content[i:i+20] == ' style="margin-botom':
                bits.append('0')
                i += 20
            else:
                i += 1
        else:
            i += 1

    return ''.join(bits)


def hide_message_in_font_tags(content, bits):
    modified_content = []
    bit_index = 0

    i = 0
    while i < len(content):
        if content[i:i+6].lower() == '<font>':
            modified_content.append('<font>')
            if bit_index < len(bits):
                if bits[bit_index] == '1':
                    modified_content.append('</font><font>')
                bit_index += 1
            modified_content.append('</font>')
            i += 6
        else:
            modified_content.append(content[i])
            i += 1

    if bit_index < len(bits):
        raise ValueError("Not enough <font> tags to hide the message")

    return ''.join(modified_content)


def extract_message_from_font_tags(content):
    bits = []
    i = 0
    while i < len(content):
        if content[i:i+6].lower() == '<font>':
            if content[i+6:i+13].lower() == '</font>':
                if content[i+13:i+19].lower() == '<font>':
                    bits.append('1')
                    i += 19
                else:
                    bits.append('0')
                    i += 13
            else:
                i += 6
        else:
            i += 1

    return ''.join(bits)


def stegano(mode, algorithm):
    if mode == '-e':
        html_cleanup("cover.html")
        message_bits = read_message("mess.txt")
        with open("cover.html", 'r') as file:
            content = file.read()

        if algorithm == '-1':
            lines = content.splitlines()
            modified_lines = hide_message_in_spaces(lines, message_bits)
            result = '\n'.join(modified_lines)
        elif algorithm == '-2':
            result = hide_message_in_double_spaces(content, message_bits)
        elif algorithm == '-3':
            result = hide_message_in_typo_attributes(content, message_bits)
        elif algorithm == '-4':
            result = hide_message_in_font_tags(content, message_bits)
        else:
            raise Exception("Usage: python3 stegano.py <-e|-d> <-1|-2|-3|-4>")

        with open("watermark.html", 'w') as file:
            file.write(result)

    elif mode == '-d':
        with open("watermark.html", 'r') as file:
            content = file.read()

        if algorithm == '-1':
            lines = content.splitlines()
            message_bits = extract_message_from_spaces(lines)
        elif algorithm == '-2':
            message_bits = extract_message_from_double_spaces(content)
        elif algorithm == '-3':
            message_bits = extract_message_from_typo_attributes(content)
        elif algorithm == '-4':
            message_bits = extract_message_from_font_tags(content)
        else:
            raise Exception("Usage: python3 stegano.py <-e|-d> <-1|-2|-3|-4>")

        hex_message = ''.join(
            f"{int(message_bits[i:i+4], 2):x}" for i in range(0, len(message_bits), 4))
        with open("detect.txt", 'w') as file:
            file.write(hex_message)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python3 stegano.py <-e|-d> <-1|-2|-3|-4>")
        sys.exit(1)

    try:
        stegano(sys.argv[1], sys.argv[2])
    except Exception as e:
        print(e)

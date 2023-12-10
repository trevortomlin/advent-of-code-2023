import re

with open("data/inputs/10.txt") as file:
    text = file.read()

regexes = [
    (r"-([S_j/])", r"_\1"),
    (r"F([S_j/])", r"f\1"),
    (r"L([S_j/])", r"l\1"),

    (r"([S_lf])-", r"\1_"),
    (r"([S_lf])7", r"\1/"),
    (r"([S_lf])J", r"\1j"),

    (r"\|(.{140}[Silj])", r"i\1"),
    (r"7(.{140}[Silj])", r"/\1"),
    (r"F(.{140}[Silj])", r"f\1"),

    (r"([Sif/].{140})\|", r"\1i"),
    (r"([Sif/].{140})L", r"\1l"),
    (r"([Sif/].{140})J", r"\1j"),
]

old = ""
while old != text:
    old = text
    for pattern, replacement in regexes:
        text = re.sub(pattern, replacement, text, flags=re.DOTALL)

# print(text.translate(str.maketrans("_if/lj.LF|-J7", "─│┌┐└┘.......")))
print("Part 1: ", len(re.findall(r"[S_ilfj/]", text)) / 2)

text = text.translate(str.maketrans("_if/lj.LF|-J7", "-|F7LJ......."))
enclosed_count = 0
for line in text.splitlines():
    line = re.sub(r"S", "|", line)
    line = re.sub(r"L-*7|F-*J", "|", line)

    print(line)

    vertical_count = 0
    for char in line:
        if char == "|":
            vertical_count += 1

        if char == "." and vertical_count % 2 == 1:
            enclosed_count += 1

print("Part 2: ", enclosed_count)
from pathlib import Path
import re

MUL_FILE_PATH = Path("src/day3/mulData.txt")
ANSWER_FILE_PATH = Path("src/day3/answer.txt")


def main():
    cleaned_items: list[str]
    sum = 0

    with open(MUL_FILE_PATH, "a+") as file:
        pattern = r"mul\(\d{1,3}\,\d{1,3}\)"
        file.seek(0)
        contents = file.read()
        cleaned_items = re.findall(pattern, contents)

        file.seek(0)
        file.truncate()
        file.writelines([f"{item}\n" for item in cleaned_items])

        pair_strs: list[list[str]] = [
            item.replace("mul(", "").replace(")", "").split(",")
            for item in cleaned_items
        ]
        num_pairs: list[tuple[int, int]] = [
            (int(pair[0]), int(pair[1])) for pair in pair_strs
        ]

        for pair in num_pairs:
            product = pair[0] * pair[1]
            sum += product

    with open(ANSWER_FILE_PATH, "w") as file:
        file.write(str(sum))


if __name__ == "__main__":
    main()

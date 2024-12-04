from pathlib import Path
import re

RAW_MUL_FILE_PATH = Path("src/day3/data/raw.txt")
ANSWER_FILE_PATH = Path("src/day3/answer.txt")
Pairs = list[str]

def main() -> None:
    reg_muls: list[str] = []
    cond_muls: list[str] = []

    try:
        with open(RAW_MUL_FILE_PATH, "r") as raw_file:
            reg_muls = findall_muleqs(raw_file)
            raw_file.seek(0)
            cond_muls = findall_muleqs(raw_file, conditional=True)

        prod_sum_reg = sum_pair_products(reg_muls)
        prod_sum_cond = sum_pair_products(cond_muls)

        with open(ANSWER_FILE_PATH, "w") as file:
            file.writelines([f"{str(prod_sum_reg)}\n", f"{str(prod_sum_cond)}"])
            print(
                f"SUCCESS! => '{prod_sum_reg:,}' written to '{ANSWER_FILE_PATH.parts[-1]}'",
                f"\nSUCCESS! => '{prod_sum_cond:,}' written to '{ANSWER_FILE_PATH.parts[-1]}'",
            )
    except FileNotFoundError as e:
        print(f"{e}")


def findall_muleqs(file, conditional=False) -> list[str]:
    contents = file.read()
    patterns = {
        "regular": r"mul\(\d{1,3}\,\d{1,3}\)",
        "enabled": r"(do\(\)|don\'t\(\)|mul\(\d{1,3},\d{1,3}\))",
    }
    cleaned_items = []

    if not conditional:
        cleaned_items = re.findall(patterns["regular"], contents)
    else:
        matches = [
            (m.group(), m.start()) for m in re.finditer(patterns["enabled"], contents)
        ]
        enabled = True

        for match, pos in matches:
            match match:
                case "do()":
                    enabled = True
                case "don't()":
                    enabled = False
                case _ if match.startswith("mul"):
                    cleaned_items.append(match) if enabled else None

    return cleaned_items


def sum_pair_products(clean_pairs: Pairs) -> int:
    str_pairs: list[list[str]] = [
        item.replace("mul(", "").replace(")", "").split(",") for item in clean_pairs
    ]
    product_sum = sum([int(pair[0]) * int(pair[1]) for pair in str_pairs])
    return product_sum


if __name__ == "__main__":
    main()

import csv
import sys
from result import Err, Ok, Result


def main(args):
    try:
        with open(args[1], "r+", newline="") as file:
            headers = args[2:]
            match make_csv(file, headers):
                case Ok(_):
                    print(f"{file} successfully reformatted")
                case Err(e):
                    print(f"an error has occured: {e}")
    except FileNotFoundError as e:
        print(e)


def make_csv(file, headers: list[str]) -> Result[None, Exception]:
    buf = file.readlines()
    writer = csv.writer(file, lineterminator="\n")
    try:
        if buf[0] != ",".join(headers) + "\n":
            file.seek(0)
            writer.writerow(headers)
            for line in buf:
                newline = [
                    word.strip()
                    for word in line.split()
                    if word and len(line.split()) > 1
                ]
                og_line = (
                    [word.strip() for word in line.split(",")] if not newline else ""
                )
                writer.writerow(newline) if newline else writer.writerow(og_line)
            file.truncate()
        else:
            raise Exception("file already properly formatted")
    except Exception as e:
        return Err(e)

    return Ok(None)


if __name__ == "__main__":
    main(sys.argv)

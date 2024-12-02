import csv
from result import Err, Ok, Result


def main():
    """Format data as csv file. It was quicker to work with this simple file in Python."""

    try:
        with open("./location_ids.csv", "r+", newline="") as file:
            match make_csv(file):
                case Ok(_):
                    print(f"{file} successfully reformatted")
                case Err(e):
                    print(f"an error has occured: {e}")
    except FileNotFoundError as e:
        print(e)


def make_csv(file) -> Result[None, Exception]:
    header1 = "listA"
    header2 = "listB"
    # Change line terminator; original data may have been in 'excel-tab' dialect
    buf = file.readlines()
    writer = csv.writer(file, lineterminator="\n")
    # Don't run if file is already formatted
    try:
        if buf[0] != f"{header1},{header2}\n":
            file.seek(0)
            writer.writerow([header1, header2])
            # Strip whitespace in line before writing row, if row not already formatted
            for line in buf:
                newline = [
                    word.strip() for word in line.split() if word and len(line.split()) > 1
                ]
                og_line = (
                    [word.strip() for word in line.split(",")] if not newline else None
                )
                writer.writerow(newline) if newline else writer.writerow(og_line)
            file.truncate()
        else:
            raise Exception("file already properly formatted")
    except Exception as e:
        return Err(e)

    return Ok(None)

if __name__ == "__main__":
    main()

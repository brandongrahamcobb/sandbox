import csv
import pathlib
import sys
import uuid


def length_extract(line):
    length = len(line)
    level = 0
    if length >= 1 and line[0] == "#":
        level += 1
        if length >= 2 and line[1] == "#":
            level += 1
            if length >= 3 and line[2] == "#":
                level += 1
    extracted_line = line[level:].replace("\n", "")
    return extracted_line, level


def build_dict_from_markdown():
    markdown_dictionary = {}
    for filename in pathlib.Path().rglob("topics.md"):
        with open(filename, "r") as mdfile:
            for ln, line in enumerate(mdfile):
                extracted_line, header_level = length_extract(line)
                markdown_dictionary[ln] = {
                    "content": extracted_line,
                    "level": header_level,
                }
                if header_level > 0:
                    markdown_dictionary[ln].update(
                        {
                            "score": {},
                        }
                    )
    return markdown_dictionary


def load_csv(markdown_dictionary):
    loaded_dictionary = markdown_dictionary
    with open(
        "scores.csv",
        "r",
    ) as file:
        r = csv.reader(file)
        y = [row for row in r]
        names = [row[0] for row in y]
        uuids = [row[1] for row in y]
        for row in y[2:]:
            for value in range(2, len(row)):
                if row[value] == "":
                    loaded_dictionary[int(row[0])]["score"][uuids[value]] = 0
                else:
                    loaded_dictionary[int(row[0])]["score"][uuids[value]] = row[value]
    return loaded_dictionary


def build_csv(uuids, names, d):
    headers_dictionary = {key: value for key, value in d.items() if value["level"] >= 1}
    data = [
        ["line_number", "topic"],
        ["", ""],
    ]
    data[0].extend(uuids)
    data[1].extend(names)
    for (
        i,
        key,
    ) in enumerate(headers_dictionary):
        before_entry = [key, "before_" + d[key]["content"]]
        after_entry = [key, "after_" + d[key]["content"]]
        for _ in names:
            before_entry.append("")
            after_entry.append("")
        data.append([])
        data[i * 2 + 2].extend(before_entry)
        data.append([])
        data[i * 2 + 3].extend(after_entry)
    with open("scores.csv", "w") as csvfile:
        w = csv.writer(csvfile)
        w.writerows(data)


def walk(updated_markdown_dictionary):
    sorted_dict = {
        k: v
        for k, v in reversed(
            sorted(
                updated_markdown_dictionary.items(),
                key=lambda item: item[1].get("average_difference", -999999999999),
            )
        )
    }
    for k, v in sorted_dict.items():
        if v["content"] == "":
            continue
        if v.get("average_difference", None) is not None:
            print(v["content"])
            for i, key in enumerate(updated_markdown_dictionary, k + 1):
                if updated_markdown_dictionary.get(i, None):
                    if updated_markdown_dictionary[i]["content"] == "":
                        continue
                    input(updated_markdown_dictionary[i]["content"])
                    if "average_difference" in updated_markdown_dictionary[i]:
                        break


def average_scores(loaded_dictionary):
    scores_dictionary = loaded_dictionary
    i = 0
    for key, data in loaded_dictionary.items():
        total = 0
        times = 0
        if data.get("score", None):
            for key, value in data.get("score", None).items():
                total += int(value)
                times += 1
            if times:
                average = total / times
                scores_dictionary[i]["average_score"] = average
        i += 1
    return scores_dictionary


def create_average_difference(markdown_dictionary, scores_dictionary):
    keys_iter = iter(scores_dictionary)
    for key in keys_iter:
        if scores_dictionary[key].get("average_score", None):
            before_score = scores_dictionary[key]["average_score"]
            next(keys_iter)
            after_score = scores_dictionary[key]["average_score"]
            difference = after_score - before_score
            markdown_dictionary[key]["average_difference"] = difference
    return markdown_dictionary


def main():

    markdown_dictionary = build_dict_from_markdown()
    match sys.argv[1]:
        case "--create":
            names = []
            uuids = []
            name = "Default"
            while name:
                name = input("Input a student's name. Insert nothing to finish.")
                if name:
                    names.append(name)
                    uuids.append(uuid.uuid4())
                    build_csv(names=names, uuids=uuids, d=d)
        case "--walkthrough":
            loaded_dictionary = load_csv(markdown_dictionary=markdown_dictionary)
            scores_dictionary = average_scores(loaded_dictionary=loaded_dictionary)
            updated_markdown_dictionary = create_average_difference(
                markdown_dictionary=markdown_dictionary,
                scores_dictionary=scores_dictionary,
            )
            walk(updated_markdown_dictionary=updated_markdown_dictionary)
        case "--before":
            before()
        case "--after":
            after()
        case _:
            raise ValueError("No arguments passed.")


if __name__ == "__main__":
    main()

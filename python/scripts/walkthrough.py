import csv
import pathlib
import pickle
import re
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


def build_dict():
    dictionary = {}
    for filename in pathlib.Path().rglob("topics.md"):
        with open(filename, "r") as mdfile:
            for ln, line in enumerate(mdfile):
                extracted_line, header_level = length_extract(line)
                dictionary[ln] = {
                    "content": extracted_line,
                    "level": header_level,
                    "before_scores": {},
                    "after_scores": {},
                }
    return dictionary


def load_csv(dictionary):
    with open(
        "scores.csv",
        "r",
    ) as file:
        r = csv.reader(file)
        header_row = None
        for i, row in enumerate(r):
            if i == 0:
                header_row = row
                continue
            if i == 1:
                continue
            for value in range(2, len(row)):
                if "before" in row[1]:
                    dictionary[int(row[0])]["before_scores"][header_row[value]] = row[
                        value
                    ]
                elif "after" in row[1]:
                    dictionary[int(row[0])]["after_scores"][header_row[value]] = row[
                        value
                    ]
    return dictionary


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


# def test_build_dict():
#     dictionary = build_dict()
#
#
# def test_build_csv():
#     dictionary = build_dict()
#     students = ["Brandon"]
#     student_uuids = ["5sfd23-a3vds3"]
#     build_csv(student_uuids=student_uuids, students=students, dictionary=dictionary)
#
#
# def test_load_csv():
#     dictionary = build_dict()
#     students = ["Brandon"]
#     student_uuids = ["5sfd23-a3vds3"]
#     build_csv(student_uuids=student_uuids, students=students, dictionary=dictionary)
#     dictionary = load_csv(dictionary)
#     print(dictionary)


def walk(dictionary):
    sorted_dict = {
        k: v
        for k, v in reversed(
            sorted(
                dictionary.items(),
                key=lambda item: item[1].get("average_difference", -999999999999),
            )
        )
    }
    for k, v in sorted_dict.items():
        if v["content"] == "":
            continue
        if v.get("average_difference", None) is not None:
            print(v["content"])
            for i, key in enumerate(dictionary, k + 1):
                if dictionary.get(i, None):
                    if dictionary[i]["content"] == "":
                        continue
                    input(dictionary[i]["content"])
                    if "average_difference" in dictionary[i]:
                        break


def average_scores(dictionary, *, score_key, average_key):
    i = 0
    for key, data in dictionary.items():
        total = 0
        times = 0
        for key, value in data.get(score_key, None).items():
            total += int(value)
            times += 1
        if times:
            average = total / times
            dictionary[i][average_key] = average
        i += 1
    return dictionary


def create_average_difference(dictionary):
    for key, value in dictionary.items():
        if value.get("average_after_score", None) and value.get(
            "average_before_score", None
        ):
            difference = (
                dictionary[key]["average_before_score"]
                - dictionary[key]["average_after_score"]
            )
            dictionary[key]["average_difference"] = difference
    return dictionary


def test_create_average_differences():
    d = build_dict()
    dicti = load_csv(dictionary=d)
    before_key = "before_scores"
    before_average_key = "average_before_score"
    after_key = "after_scores"
    after_average_key = "average_after_score"
    dictio = average_scores(
        dictionary=dicti, score_key=before_key, average_key=before_average_key
    )
    dictio.update(
        average_scores(
            dictionary=dictio,
            score_key=after_key,
            average_key=after_average_key,
        )
    )
    diction = create_average_difference(dictionary=dictio)
    walk(dictionary=diction)


def main():

    d = build_dict()
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
            dicti = load_csv(dictionary=d)
            before_key = "before_scores"
            before_average_key = "average_before_score"
            after_key = "after_scores"
            after_average_key = "average_after_score"
            dictio = average_scores(
                dictionary=dicti, score_key=before_key, average_key=before_average_key
            )
            dictio.update(
                average_scores(
                    dictionary=dictio,
                    score_key=after_key,
                    average_key=after_average_key,
                )
            )
            diction = create_average_difference(dictio)
            walk(dictionary=diction)
        case "--before":
            before()
        case "--after":
            after()
        case _:
            raise ValueError("No arguments passed.")


if __name__ == "__main__":
    main()

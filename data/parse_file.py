import json
with open("data/emoji.json") as file_obj:
    count = 0
    for x in json.load(file_obj):
        count += 1
        print(count, x['emoji'])

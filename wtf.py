import json
import time

start = time.time()
dataSet = json.load(open("sample.json"))

gps_exists = set()
newData = []
for data in dataSet:
    gpsData = data["GPS"]
    gps_tuple = (str(gpsData["latitude"]), str(gpsData["longitude"]))
    if gps_tuple in gps_exists:
        continue
    else:
        newData.append(data)
        gps_exists.add(gps_tuple)

with open("sample-python.json", "w") as f:
    json.dump(newData, f, indent=2)

end = time.time()
print(f"Elapsed (python): {(end-start)*1000} milliseconds")

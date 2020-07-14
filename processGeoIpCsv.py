import csv, os, json, shutil
from math import sqrt, floor, ceil

RAW_DATABASE_DIR = "raw"
DATA_DIR = "data"
CODE_DIR = "build"
PARAMS_FILE = os.path.join(CODE_DIR, "params.js")
BLOCK_SIZE = 2**12 # = 4KB, the storage block size on almost all new OSes
FILE_SIZE = BLOCK_SIZE*12 - 100 # File size is made to be lower than the size of 12 storage blocks (minus a few bytes to account for overheads) in order to make sure that all the file's contents are directly addressed from the file's inode (preventing indirect access to storage blocks)

def removeOldData():
    shutil.rmtree(DATA_DIR, ignore_errors=True) # Clean directory
    os.mkdir(DATA_DIR)
    try:
        os.mkdir(CODE_DIR)
        os.remove(PARAMS_FILE)
    except (OSError):
        pass

def jsonify(item):
    return json.dumps(item).encode('utf-8')

def storeFile(filename, content, binary = False):
    if binary:
        mode = "wb"
    else:
        mode = "w"
    with open(os.path.join(DATA_DIR, filename), mode) as newFile:
        newFile.write(content)

def parseNumber(num, parser):
    return 0 if num=="" else parser(num)

def extract_location_attrs(row):
    # The only data from the locations file that is returned by neode-geoip is:
    # - country_iso_code (row 4)
    # - subdivision_1_iso_code (row 6)
    # - city_name (row 10)
    # - metro_code (row 11)
    # - time_zone (row 12)
    # - is_in_european_union (row 13)
    return [row[4], row[6],row[10], parseNumber(row[11], int), row[12], row[13] ]

def generateLocationsFile():
    geoname_ids = {}
    location_items= []
    max_item_length=0
    with open(os.path.join(RAW_DATABASE_DIR, "GeoLite2-City-Locations-en.csv")) as locations_file:
        locations = csv.reader(locations_file, delimiter=',')
        next(locations) # Ignore first line (headers)
        counter = 0
        for row in locations:
            current_geoname_id = row[0]
            geoname_ids[current_geoname_id]=counter
            counter+=1
            stored_attrs = jsonify(extract_location_attrs(row))
            location_items.append(stored_attrs)
            max_item_length=max(max_item_length, len(stored_attrs))

    location_items=map(lambda item: item.rjust(max_item_length, b' '), location_items)
    new_location_file_content = b'['+b','.join(location_items)+b']' # Make it into a json even if it will not be used that way
    storeFile("locations.json", new_location_file_content, True)

    return [geoname_ids, max_item_length+1]

def extract_block_attrs(row, geoname_ids):
    # Attrs used by node-geoip:
    # - range (will be derived from the ip being searched) [0]
    # - geoname_id (needs to be transformed to match the ids generated before for the locations file) [1]
    # - latitude [7]
    # - longitude [8]
    # - accuracy_radius [9]
    try:
        locations_id = geoname_ids[row[1]]
    except:
        locations_id = geoname_ids.get(row[2], None)
    return [ locations_id, parseNumber(row[7], float), parseNumber(row[8], float), parseNumber(row[9], int)]

def storeIps(ips, counter, ipIndex):
    ips = ips[:-1] + b']' # Remove the trailing comma and add ]
    ipIndex.append(json.loads(ips.decode('utf-8'))[0][0]) # Store the first IP of the set into the index
    storeFile("%d.json" % counter, ips, True)

def ipStr2Int(strIp):
    ip = [int(e) for e in strIp.split('.')]
    return ip[0]*256**3 + ip[1]*256**2 + ip[2]*256**1 + ip[3]

def generateBlockFiles(geoname_ids):
    counter = 0
    ips = b'['
    ipIndex = []
    with open(os.path.join(RAW_DATABASE_DIR, "GeoLite2-City-Blocks-IPv4.csv")) as blocks_file:
        blocks = csv.reader(blocks_file, delimiter=',')
        next(blocks) # Skip headers
        for row in blocks:
            [ip, mask] = row[0].split('/')
            mask = int(mask)
            ip = ipStr2Int(ip)
            attrs = jsonify([ip] + extract_block_attrs(row, geoname_ids)) + b','
            if len(ips + attrs) > FILE_SIZE:
                storeIps(ips, counter, ipIndex)
                counter += 1
                ips = b'[' + attrs
            else:
                ips += attrs

    storeIps(ips, counter, ipIndex)
    return ipIndex

def generateIndexes(ipIndex):
    rootIpIndex = []
    ROOT_NODES = int(floor(sqrt(len(ipIndex)))) # See readme for the rationale behind this formula
    MID_NODES = int(ceil(len(ipIndex)/ROOT_NODES))
    for i in range(ROOT_NODES):
        rootIpIndex.append(ipIndex[i*MID_NODES])
        storeFile("i%d.json" % i, json.dumps(ipIndex[i*MID_NODES:(i+1)*MID_NODES]))

    storeFile("index.json", json.dumps(rootIpIndex))
    return MID_NODES

def storeDynamicParams(location_record_length, num_mid_nodes):
    with open(PARAMS_FILE, "w") as params_file:
        params = {
                "LOCATION_RECORD_SIZE": location_record_length,
                "NUMBER_NODES_PER_MIDINDEX": num_mid_nodes
            }
        params_file.write("module.exports = " + json.dumps(params, indent=4)) # Pretty-printed json

def main():
    removeOldData()
    [geoname_ids, location_record_length] = generateLocationsFile()
    ipIndex = generateBlockFiles(geoname_ids)
    num_mid_nodes = generateIndexes(ipIndex)
    storeDynamicParams(location_record_length, num_mid_nodes)

if __name__ == '__main__':
    main()

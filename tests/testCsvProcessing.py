import unittest
import json, shutil

# Import file from the parent directory
# Taken from https://stackoverflow.com/a/11158224
import os,sys,inspect
currentdir = os.path.dirname(os.path.abspath(inspect.getfile(inspect.currentframe())))
parentdir = os.path.dirname(currentdir)
sys.path.insert(0,parentdir)
import processGeoIpCsv as geoip

def buildFilePath(filename, parentFolder):
    return filename if  parentFolder == None else os.path.join(parentFolder, filename)

def writeFile(contents, filename, parentFolder = None):
    filepath = buildFilePath(filename, parentFolder)
    with open(filepath, "w") as fileToWrite:
        fileToWrite.write(contents)

def readFile(filename, parentFolder = None, jsonParsed = True):
    filepath = buildFilePath(filename, parentFolder)
    with open(filepath) as fileToRead:
        content = fileToRead.read()
        if jsonParsed:
            return json.loads(content)
        else:
            return content

def createLocationsFile():
    writeFile(
    """header1, header2, header3...
5819,en,EU,Europe,CY,Cyprus,02,Limassol,,,Souni,,Asia/Nicosia,1
6252001,en,NA,"North America",US,"United States",,,,,,,America/Chicago,0
""",
    "GeoLite2-City-Locations-en.csv",
    "raw")

def createBlocksFile():
    # Create a file with the following special cases:
    #   - Normal record
    #   - Record without geoname_id
    #   - Record without longitude, latitude nor range
    #   - Record without geoname_id nor registered_country_geoname_id
    #   - Some extra records to cause the algorithm to split this into multiple files
    writeFile(
    """header1, header2, header3...
207.97.192.0/18,6252001,6252001,,0,0,,37.7510,-97.8220,1000
23.161.144.0/20,,6252001,,0,0,,,,
80.231.5.0/24,,,,0,1,,,,
""" + "208.69.72.0/21,,6252001,,0,0,,,,\n"*2000,
    "GeoLite2-City-Blocks-IPv4.csv",
    "raw")

DATA_DIRECTORIES = [geoip.RAW_DATABASE_DIR, geoip.DATA_DIR, geoip.CODE_DIR]

class TestDataGenerator(unittest.TestCase):
    def setUp(self):
        for directory in DATA_DIRECTORIES:
            try:
                os.mkdir(directory)
            except OSError:
                pass

    def tearDown(self):
        for directory in DATA_DIRECTORIES:
            shutil.rmtree(directory, ignore_errors=True)


    def test_ipStr2Int(self):
        self.assertEqual(geoip.ipStr2Int("0.0.1.0"), 256)
        self.assertEqual(geoip.ipStr2Int("0.0.0.244"), 244)

    def test_generateLocationsFile(self):
        createLocationsFile()
        [geonames_dict, record_length] = geoip.generateLocationsFile()
        self.assertDictEqual(geonames_dict, {"5819": 0, "6252001": 1})
        processedLocations = readFile("locations.json", geoip.DATA_DIR, False)
        self.assertEqual(len(processedLocations), record_length*2 + len("[") + len("]") - len(","))
        self.assertListEqual(
            json.loads(processedLocations),
            [
                ["CY", "02", "Souni", 0, "Asia/Nicosia", "1"],
                ["US", "", "", 0, "America/Chicago", "0"]
            ])

    def test_generateBlockFiles(self):
        createBlocksFile()
        geonames = {"6252001": 1}
        ipIndex = geoip.generateBlockFiles(geonames)
        # Ip index is constructed properly
        firstIp = geoip.ipStr2Int("207.97.192.0")
        secondBlockIp = geoip.ipStr2Int("208.69.72.0")
        self.assertListEqual(ipIndex, [firstIp, secondBlockIp])
        for i in range(len(ipIndex)):
            self.assertEqual(readFile("%d.json" % i, "data")[0][0], ipIndex[i])
        firstBlock = readFile("0.json", "data")
        # Numerical values are recorded properly
        record = firstBlock[0]
        self.assertListEqual(record, [geoip.ipStr2Int("207.97.192.0"), geonames["6252001"], 37.7510,-97.8220,1000])
        # Records with missing numerical values such as range or latitude have these values replaced by zeroes
        record = firstBlock[3]
        self.assertListEqual(record, [geoip.ipStr2Int("208.69.72.0"), geonames["6252001"], 0,0,0])
        # Records without geoname_id take the id of the country
        record = firstBlock[1]
        self.assertTrue(record[0]==geoip.ipStr2Int("23.161.144.0") and record[1]==geonames["6252001"])
        # Records without geoname_id nor country_id have their id set to null
        record = firstBlock[2]
        self.assertTrue(record[0]==geoip.ipStr2Int("80.231.5.0") and record[1]==None)

    def test_generateIndexes(self):
        geoip.generateIndexes([1])
        for filename in ["i0.json", "index.json"]:
            self.assertListEqual(readFile(filename, geoip.DATA_DIR), [1])

    def test_storeDynamicParams(self):
        params = {
                "LOCATION_RECORD_SIZE": 420,
                "NUMBER_NODES_PER_MIDINDEX": 69
                }
        geoip.storeDynamicParams(
                params["LOCATION_RECORD_SIZE"],
                params["NUMBER_NODES_PER_MIDINDEX"]
            )
        self.assertDictEqual(
                json.loads(readFile(geoip.PARAMS_FILE, jsonParsed=False)[len("module.exports = "):]),
                params
            )

    def test_all(self):
        createLocationsFile()
        createBlocksFile()
        geoip.main()
        # Just check that:
        # - The basic files are generated
        # - Generated files are valid jsons
        # - No exception is thrown by the program
        for filename in ["0.json", "locations.json", "i0.json", "index.json"]:
            readFile(filename, geoip.DATA_DIR)

if __name__ == '__main__':
    unittest.main()

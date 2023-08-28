import urllib.request, os, zipfile, glob, shutil
import processGeoIpCsv as geoip

ZIP_FILENAME = "geolite.zip"
TEMPORAL_EXTRACTED_DIR = "geoip"

def rmtree(directory):
    shutil.rmtree(directory, ignore_errors=True)

urllib.request.urlretrieve ("https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City-CSV&suffix=zip&license_key=" + os.environ["MAXMIND_LICENSE_KEY"], ZIP_FILENAME)

with zipfile.ZipFile(ZIP_FILENAME, 'r') as zip_ref:
    zip_ref.extractall(TEMPORAL_EXTRACTED_DIR)

rmtree(geoip.RAW_DATABASE_DIR)

extracted_dir = glob.glob('./'+TEMPORAL_EXTRACTED_DIR+'/GeoLite2-City-CSV_[0-9]*')[0]
os.rename(extracted_dir, geoip.RAW_DATABASE_DIR)

rmtree(TEMPORAL_EXTRACTED_DIR)

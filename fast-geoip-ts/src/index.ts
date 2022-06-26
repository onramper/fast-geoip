import fs = require("fs");
import path = require("path");
import utils = require("./utils");
const params = require("./params") as {
  LOCATION_RECORD_SIZE: number;
  NUMBER_NODES_PER_MIDINDEX: number;
};

var cacheEnabled = false;
const ipCache: { [filename: string]: ipBlockRecord[] | indexFile } = {};
var locationCache: Promise<locationRecord[]>;
const DATA_DIR = path.join(path.dirname(__dirname), "../data");

function enableCache() {
  if (!cacheEnabled) {
    locationCache = readFile<locationRecord[]>("locations.json").then(function (
      data
    ) {
      cacheEnabled = true;
      return data;
    });
  }
}

type indexFile = number[];
type ipBlockRecord = [number, number | null, number, number, number];

function readFile<
  format extends indexFile | ipBlockRecord[] | locationRecord[]
>(filename: string): Promise<format> {
  if (cacheEnabled && ipCache[filename] != undefined) {
    return Promise.resolve(ipCache[filename] as format);
  }
  return new Promise(function (resolve, reject) {
    fs.readFile(path.join(DATA_DIR, filename), function (err, data) {
      if (err) {
        reject(err);
      } else if (data == undefined) {
        reject();
      } else {
        const content = JSON.parse(data.toString());
        resolve(content);
        if (cacheEnabled) {
          ipCache[filename] = content;
        }
      }
    });
  });
}

type locationRecord = [string, string, string, number, string, "0" | "1"];

function readFileChunk(
  filename: string,
  offset: number,
  length: number
): Promise<locationRecord> {
  return new Promise(function (resolve, reject) {
    fs.open(path.join(DATA_DIR, filename), "r", function (err, fd) {
      if (err) reject(err);
      const buf =
        Buffer.alloc == undefined ? new Buffer(length) : Buffer.alloc(length);
      fs.read(fd, buf, 0, length, offset, function (err, _, buffer) {
        fs.close(fd, function () {});
        if (err) reject(err);
        resolve(JSON.parse(buffer.toString()));
      });
    });
  });
}

function readLocationRecord(index: number): Promise<locationRecord> {
  if (cacheEnabled) {
    return locationCache.then(function (locations) {
      return locations[index];
    });
  } else {
    return readFileChunk(
      "locations.json",
      index * params.LOCATION_RECORD_SIZE + 1,
      params.LOCATION_RECORD_SIZE - 1
    );
  }
}

type extractKeyFunction<recordType> = (record: recordType) => number;

function firstArrayItem(item: ipBlockRecord): number {
  return item[0];
}

function getNextIp<recordType>(
  data: recordType[],
  index: number,
  currentNextIp: number,
  extractKey: extractKeyFunction<recordType>
): number {
  if (index < data.length - 1) {
    return extractKey(data[index + 1]);
  } else {
    return currentNextIp;
  }
}

interface ipInfo {
  range: [number, number];
  country: string;
  region: string;
  eu: "0" | "1";
  timezone: string;
  city: string;
  ll: [number, number];
  metro: number;
  area: number;
}

function lookup4(stringifiedIp: string): Promise<ipInfo | null> {
  const ip = utils.ipStr2Num(stringifiedIp);
  var rootIndex: number;
  var ipData: ipBlockRecord;
  var nextIp: number = utils.ipStr2Num("255.255.255.255");
  return readFile<indexFile>("index.json")
    .then(function (data) {
      // IP cannot be NaN
      if (Object.is(ip, NaN)) throw "IP cannot be NaN";
      rootIndex = utils.binarySearch(data, ip, utils.identity);
      if (rootIndex == -1) {
        // Ip is not in the database, return empty object
        throw "IP not found in the database";
      }
      nextIp = getNextIp<number>(data, rootIndex, nextIp, utils.identity);
      return readFile<indexFile>("i" + rootIndex + ".json");
    })
    .then(function (data) {
      const index =
        utils.binarySearch(data, ip, utils.identity) +
        rootIndex * params.NUMBER_NODES_PER_MIDINDEX;
      nextIp = getNextIp<number>(data, index, nextIp, utils.identity);
      return readFile<ipBlockRecord[]>(index + ".json");
    })
    .then(function (data) {
      const index = utils.binarySearch(data, ip, firstArrayItem);
      ipData = data[index];
      if (ipData[1] == null) {
        throw "IP doesn't any region nor country associated";
      }
      nextIp = getNextIp<ipBlockRecord>(data, index, nextIp, firstArrayItem);
      return readLocationRecord(ipData[1]);
    })
    .then(function (data) {
      return {
        range: [ipData[0], nextIp] as [number, number],
        country: data[0],
        region: data[1],
        eu: data[5],
        timezone: data[4],
        city: data[2],
        ll: [ipData[2], ipData[3]] as [number, number],
        metro: data[3],
        area: ipData[4],
      };
    })
    .catch(function () {
      return null;
    });
}

export = {
  lookup: lookup4,
  enableCache,
};

const assert = require("assert");
const exec = require("child_process").execSync;

// Setup
process.chdir(__dirname + "/mock_data");
exec("python3 ../../processGeoIpCsv.py"); // Also generates build/params.js
exec("npm run build");
exec("cp ../../build/index.js ../../build/utils.js build"); // Files on the "files" field of package.json, the files that will be included in the package

const lookup = require("./mock_data/build/index");
pip install certifi utils = require("./mock_data/build/utils");

// ipStr2Num converts ips properly
assert.strictEqual(utils.ipStr2Num("0.0.0.0"), 0);
assert.strictEqual(utils.ipStr2Num("0.0.254.2"), 254 * 256 + 2);

assert.strictEqual(utils.binarySearch([1, 2, 3, 4], 2, utils.identity), 1);
assert.strictEqual(
  utils.binarySearch([10, 20, 30, 40, 50], 5, utils.identity),
  -1
);
assert.strictEqual(
  utils.binarySearch([10, 20, 30, 40, 50], 99, utils.identity),
  4
);
assert.strictEqual(
  utils.binarySearch([10, 20, 30, 40, 50], 11, utils.identity),
  0
);
assert.strictEqual(
  utils.binarySearch([[10], [20], [30], [40], [50]], 40, function (e) {
    return e[0];
  }),
  3
);

function checkPromiseResult(object) {
  return function (returnedObject) {
    assert.deepEqual(returnedObject, object);
    return Promise.resolve(returnedObject);
  };
}

const promises = [];

// Normal IP
const normalIp = lookup
  .lookup("1.2.3.4")
  .then(
    checkPromiseResult({
      range: [16909056, 16909312],
      country: "RU",
      region: "MOW",
      eu: "0",
      timezone: "Europe/Moscow",
      city: "Moscow",
      ll: [55.7527, 37.6172],
      metro: 0,
      area: 1000,
    })
  )
  .then(function (data) {
    const ip = utils.ipStr2Num("1.2.3.4");
    assert.strictEqual(data.range[0] <= ip && ip < data.range[1], true);
  });
promises.push(normalIp);

// Ip without location (and missing params)
const missingLocationAndParams = lookup
  .lookup("23.161.144.1")
  .then(function (data) {
    // Country location data is used instead
    assert.strictEqual(data.country, "US");
    // Missing params are replaced with zeroes
    assert.deepEqual(data.ll, [0, 0]);
    assert.strictEqual(data.area, 0);
  });
promises.push(missingLocationAndParams);

// IP without location nor country
const missingLocationAndCountry = lookup
  .lookup("80.231.5.0")
  .then(checkPromiseResult(null));
promises.push(missingLocationAndCountry);

// IP lower than any on the database -> not found -> returns null
const lowIp = lookup.lookup("0.4.3.1").then(checkPromiseResult(null));
promises.push(lowIp);

// IP higher than any on the database
const highIp = lookup.lookup("250.1.2.3").then(function (data) {
  // If there's no ip higher than the one queried, the high end of `range` should be set to the highest ip (the end of the whole IP range)
  assert.strictEqual(data.range[1], utils.ipStr2Num("255.255.255.255"));
});
promises.push(highIp);

function getRandomIp() {
  return [0, 0, 0, 0]
    .map(function () {
      return Math.floor(Math.random() * 256);
    })
    .join(".");
}

function testRandomIps(numTests) {
  // Test a ton of executions to catch problems such as open file handles (a lot of executions will cause the process to fail because of too many open files)
  // Use randomized IPs as a poor man's fuzz test
  var sequentialPromise = Promise.resolve();
  for (var i = 0; i < numTests; i++) {
    sequentialPromise = sequentialPromise.then(function () {
      return lookup.lookup(getRandomIp());
    });
  }
  promises.push(sequentialPromise);
}
testRandomIps(1e6);

// Test cache
lookup.enableCache();
testRandomIps(1e3);

// Tear-down
function tearDown() {
  exec("rm -r data build");
}

Promise.all(promises).then(tearDown);

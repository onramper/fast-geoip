const geoip = require("../build");

(async () => {
  const location = await geoip.lookup("81.22.36.183");

  console.log(location);
})();

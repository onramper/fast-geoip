type extractKeyFunction<recordType> = (record: recordType) => number

function identity(item: number): number {
  return item
}

function binarySearch<recordType>(list: recordType[], item: number, extractKey: extractKeyFunction<recordType>): number {
  var low = 0;
  var high = list.length - 1;
  while (true) {
    var i = Math.round((high - low) / 2) + low
    if (item < extractKey(list[i])) {
      if (i == high && i == low) {
        return - 1 // Item is lower than the first item
      } else if (i == high) {
        high = low
      } else {
        high = i;
      }
    } else if (item >= extractKey(list[i]) && (i == (list.length - 1) || item < extractKey(list[i + 1]))) {
      return i;
    } else {
      low = i;
    }
  }
}

function ipStr2Num(stringifiedIp: string): number {
  return stringifiedIp.split('.')
    .map(
      function (e) {
        return parseInt(e)
      })
    .reduce(
      function (acc, val, index) {
        return acc + val * Math.pow(256, 3 - index)
      }
      , 0)
}

export = {
  binarySearch,
  identity,
  ipStr2Num
}

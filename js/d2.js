console.log('day2');

const fs = require('fs')

//const input = fs.readFileSync('d2.test.input', 'utf8')
const input = fs.readFileSync('d2.input', 'utf8')
  .toString()
  .trim()
  .split(/[\r\n]+/)
  .map(i => i.split(/\s+/))
  .map(i => ({
    direction: i[0],
    distance: Number(i[1]),
  }))

let h = 0
let d = 0
let aim = 0
let haim = 0
let daim = 0

for (const {direction, distance} of input) {
  switch (direction) {
      case "forward":
          h += distance
          haim += distance
          daim += aim * distance
          break;
      case "down":
          aim += distance
          d += distance
          break;
      case "up":
          aim -= distance
          d -= distance
          break;
  }
}

console.log(`horizontal ${h} * depth ${d} = ${h * d}`)
console.log(`horizontal ${haim} * depth ${daim} = ${haim * daim}`)

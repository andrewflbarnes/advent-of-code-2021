console.log('day1');

const fs = require('fs')

const input = fs.readFileSync('d1.input', 'utf8')
  .toString()
  .trim()
  .split(/[\r\n]+/)
  .map(i => Number(i))

let inc = 0;
let incWindow = 0;
let last = input[0]
let lastWIndow = input[0]
for (i = 1; i < input.length; i++) {
    const next = input[i]
    if (last < next) {
        inc++
    }
    if (i >= 3) {
        if (lastWindow < next) {
            incWindow++
        }
    }
    last = next
    lastWindow = input[i-2]
}

console.log(inc)
console.log(incWindow)

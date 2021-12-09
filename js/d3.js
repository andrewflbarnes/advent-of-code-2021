console.log('day3');

const fs = require('fs')

//const input = fs.readFileSync('d3.test.input', 'utf8')
const input = fs.readFileSync('d3.input', 'utf8')
  .toString()
  .trim()
  .split(/[\r\n]+/)
  .map(b => [...b])
  .map(b => b.map(i => Number(i)))

const inputSize = input[0].length
const initArr = () => Array(inputSize).fill(0)

const res = input
  .reduce((acc, next) => {
    next.forEach((v, i) => {
      const delta = v == 0 ? -1 : 1
      acc[0][i] += delta
      acc[1][i] -= delta
    })
    return acc
  }, [initArr(), initArr()])
  .map(acc => acc.map(i => i > 0 ? 1 : 0))
  .map(binArr => binArr.join(''))
  .map(bin => parseInt(bin, 2))
  .reduce((acc, next) => acc * next)

console.log(res)

console.log(generatorRating(input, true) * generatorRating(input, false))

function generatorRating(input, oxygen) {
  let pos = 0
  let process = input
  while (true) {
    let bitVal = process
      .map(b => b[pos])
      .reduce((acc, next) => acc + (next == 0 ? -1 : 1), 0)
  
    if (oxygen) {
      bitVal = bitVal >= 0 ? 1 : 0
    } else {
      bitVal = bitVal >= 0 ? 0 : 1
    }
 
    process = process.filter(val => val[pos] == bitVal)
  
    if (process.length == 1) {
      return parseInt(process[0].join(''), 2)
    }
  
    pos += 1
  }
}

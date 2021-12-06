const fs = require("fs");
const path = require("path");
const readline = require("readline");

const reader = readline.createInterface({
  input: fs.createReadStream(path.join(__dirname, "../input.txt")),
  crlfDelay: Infinity,
});
const measurements = [];

reader.on("line", (line) => measurements.push(parseInt(line, 10)));
reader.on("close", () => {
    let i = 0;
    let increasementCounter = 0;
    while (i + 4 <= measurements.length) {
        let valA = measurements.slice(i, i + 3).reduce((sum, val) => sum + val, 0);
        let valB = measurements.slice(i + 1, i + 4).reduce((sum, val) => sum + val, 0);
        if (valB > valA) {
            increasementCounter++;
        }
        i++;
    }
    console.log(`There is ${increasementCounter} increasements`);
});

const fs = require("fs");
const path = require("path");
const readline = require("readline");

const reader = readline.createInterface({
  input: fs.createReadStream(path.join(__dirname, "../input.txt")),
  crlfDelay: Infinity,
});
const position = {
  horizontal: 0,
  depth: 0,
};
let currentAim = 0;

reader.on("line", (line) => {
  const [direction, velocity] = line.split(" ");
  const instruction = {
    direction,
    velocity: parseInt(velocity, 10)
  };
  switch (instruction.direction) {
    case "forward":
      position.horizontal += instruction.velocity;
      position.depth += currentAim * instruction.velocity;
      break;
    case "down":
      currentAim += instruction.velocity;
      break;
    case "up":
      currentAim -= instruction.velocity;
      break;
    default:
      throw new Error(`Unknown direction ${direction}`);
  }
});

reader.on("close", () => {
  console.log(
    `Result is ${position.depth * position.horizontal} for position`,
    position
  );
});

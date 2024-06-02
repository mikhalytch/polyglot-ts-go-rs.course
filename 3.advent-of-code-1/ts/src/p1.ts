

function getInput(): string {
	return `forward 5 
down 5 
forward 8 
up 3 
down 8 
forward 2 `
}

// parseLine returns line's numbers representation, [delta-y, delta-x]
function parseLine(line: string): [number, number] {
	// return [0, 0]
	const [dir, a] = line.split(" ");
	const amount = +a;

	if (dir === "forward") {
		return [0, amount];
	} else if (dir === "up") {
		return [amount, 0];
	}
	return [-amount, 0];
}

const items = getInput().
	split("\n").
	map(x => parseLine(x));
// console.log(items);
const resultPositionYX = items.
	reduce((acc, dirAmount) => {
		acc[0] += dirAmount[0]; // dir
		acc[1] += dirAmount[1]; // amount
		return acc;
	}, [0, 0]);
const result = resultPositionYX[0] * resultPositionYX[1];

console.log(resultPositionYX, result);


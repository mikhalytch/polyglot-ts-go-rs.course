function getInput(): string {
	return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;
}

enum Thing { Tree, Snow, }

const things = getInput().
	split("\n").
	map(x => x.split("").map(x => x === "#" ? Thing.Tree : Thing.Snow))

const colLen = things[0].length

let treeCount = 0;

things.forEach(thingRow => {

})

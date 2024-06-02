/*
const a = [];
const b = a;

b.push(1);

console.log(a); */

/* type Foo = () => void

enum TSEnum {
	Foo,
	Bar,
	Baz
}
*/

type Foo = {
	bar?: string;
}

function foSmth(foo: Foo): boolean {
	if (foo.bar) {
		return true;
	} else {
		return false;
	}

}

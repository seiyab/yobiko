export function counter() {
	let current = 0;
	return { next };
	function next(): number {
		return current++;
	}
}

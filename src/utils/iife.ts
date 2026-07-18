export function iife<T>(f: () => T): T {
	return f();
}

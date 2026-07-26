type State<T> = {
	getSnapshot: () => T;
	subscribe: (listener: () => void) => () => void;
	update: (action: (prev: T) => T) => void;
};

export function newState<T>(initial: T): State<T> {
	let state: T = initial;
	const subscriptions = new Set<() => void>();

	return {
		getSnapshot,
		subscribe,
		update,
	};

	function getSnapshot() {
		return state;
	}

	function subscribe(listener: () => void): () => void {
		subscriptions.add(listener);
		return () => subscriptions.delete(listener);
	}

	function update(action: (prev: T) => T): void {
		let old = state;
		state = action(old);
		if (old !== state) {
			emit();
		}
	}

	function emit() {
		for (const s of subscriptions) {
			s();
		}
	}
}

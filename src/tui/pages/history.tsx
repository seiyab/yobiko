import { runner } from "#app/core/runner.js";
import { useSyncExternalStore } from "react";

function History() {
	const snapshot = useSyncExternalStore(
		runner.state.subscribe,
		runner.state.getSnapshot,
	);
}

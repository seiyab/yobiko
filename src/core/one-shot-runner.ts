import { Task } from "./provider.js";
import { spawnSync } from "node:child_process";

function newOneShotRunner() {
	let task: Task | null = null;

	return { register, maybeAttachAndRun };

	function register(t: Task): void {
		task = t;
	}

	function maybeAttachAndRun(): void {
		if (task == null) return;

		spawnSync(task.command, task.args ?? [], {
			cwd: task.cwd,
			stdio: "inherit",
		});
	}
}

export const { register, maybeAttachAndRun } = newOneShotRunner();

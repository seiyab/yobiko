import { counter } from "#app/utils/counter.js";
import { Task } from "./provider.js";
import { spawn as spawnProcess } from "node:child_process";

type Run = {
	id: string;
	task: Task;
	status: RunStatus;
	createdAt: Date;
};

export type RunStatus = "running" | "failed" | "succeeded";

type Spawn = (t: Task) => void;

type RunStore = {
	subscribe: (listener: () => void) => () => void;
	getSnapshot: () => Run[];
};

function newRunner(): { spawn: Spawn; state: RunStore } {
	const sc = counter();
	const subscriptions = new Map<number, () => void>();

	const rc = counter();
	let runs: Run[] = [];

	return {
		spawn,
		state: { subscribe, getSnapshot },
	};

	function spawn(task: Task): void {
		const id = String(rc.next());
		const p = spawnProcess(task.command, task.args ?? [], { cwd: task.cwd });
		p.addListener("exit", (code) => {
			runs = runs.map((run) => {
				if (run.id !== id) return run;
				return {
					...run,
					status: code === 0 ? "succeeded" : "failed",
				};
			});
			emit();
		});

		runs = runs.concat([
			{
				id,
				task,
				status: "running",
				createdAt: new Date(),
			},
		]);
		emit();
	}

	function subscribe(listener: () => void): () => void {
		const i = sc.next();
		subscriptions.set(i, listener);
		return () => subscriptions.delete(i);
	}

	function getSnapshot(): Run[] {
		return runs;
	}

	function emit() {
		for (const listener of subscriptions.values()) {
			listener();
		}
	}
}

export const runner = newRunner();

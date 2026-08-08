import { counter } from "#app/utils/counter.js";
import { Task } from "./provider.js";
import { spawn as spawnProcess } from "node:child_process";

export type RunOutput = {
	subscribe: (listener: () => void) => () => void;
	getSnapshot: () => string;
};

type Run = {
	id: string;
	task: Task;
	status: RunStatus;
	createdAt: Date;
	output: RunOutput;
	kill: () => void;
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
		const output = newOutput();
		const p = spawnProcess(task.command, task.args ?? [], { cwd: task.cwd });
		p.stdout.setEncoding("utf8");
		p.stderr.setEncoding("utf8");
		p.stdout.addListener("data", output.append);
		p.stderr.addListener("data", output.append);
		p.addListener("exit", (code) => {
			runs = runs.map((run) => {
				if (run.id !== id) return run;
				return {
					...run,
					status: code === 0 ? "succeeded" : "failed",
					kill: () => 0,
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
				output: output.state,
				kill: p.kill.bind(p),
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

function newOutput(): { state: RunOutput; append: (chunk: string) => void } {
	const subscriptions = new Set<() => void>();
	let output = "";

	return {
		state: { subscribe, getSnapshot },
		append,
	};

	function subscribe(listener: () => void): () => void {
		subscriptions.add(listener);
		return () => subscriptions.delete(listener);
	}

	function getSnapshot(): string {
		return output;
	}

	function append(chunk: string): void {
		output += chunk;
		for (const listener of subscriptions) listener();
	}
}

export const runner = newRunner();

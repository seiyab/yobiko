import { PathLike } from "node:fs";

export type Provider = {
	name: string;
	matches: (p: PathLike) => Promise<boolean>;
	tasks: (p: PathLike) => Promise<Task[]>;
};

export type Task = {};

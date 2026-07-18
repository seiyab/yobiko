import { PathLike } from "node:fs";

export type Provider = {
	name: string;
	workspace: (p: PathLike) => Promise<Workspace | null>;
};

export type Workspace = {
	dir: string;
	provider: Provider;
	tasks: Task[];
};

export type Task = {
	name: string;
	cwd: string;
	command: string;
	content?: string;
};

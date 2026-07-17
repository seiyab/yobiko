import { PathLike } from "node:fs";
import { Provider, Task } from "#app/core/provider.js";
import { promises as fs } from "node:fs";
import path from "node:path";

async function matches(p: PathLike): Promise<boolean> {
	return (await fs.readdir(p)).includes("package.json");
}

async function tasks(p: PathLike): Promise<Task[]> {
	return [];
}

export const npm: Provider = { name: "npm", matches, tasks };

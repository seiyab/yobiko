import { PathLike } from "node:fs";
import { Provider, Task, Workspace } from "#app/core/provider.js";
import { promises as fs } from "node:fs";
import path from "node:path";

export const npm: Provider = { name: "npm", workspace };

async function workspace(p: PathLike): Promise<Workspace | null> {
	const entries = await fs.readdir(p);
	if (!entries.includes("package.json")) return null;
	const pkg = JSON.parse(await fs.readFile(path.join(p.toString(), "package.json"), "utf8"));
	const r = await runner(p);
	const scripts = pkg.scripts;
	const tasks = Object.entries(scripts).map(
		([script, value]): Task => ({
			name: script,
			cwd: p.toString(),
			content: String(value),
			...r(script),
		}),
	);
	return {
		dir: p.toString(),
		provider: npm,
		tasks,
	};
}

type Runner = (script: string) => {
	command: string;
	args: string[];
};
async function runner(p: PathLike): Promise<Runner> {
	const entries = new Set(await fs.readdir(p));
	if (entries.has("pnpm-lock.yaml"))
		return (script) => ({ command: "pnpm", args: ["run", script] });
	if (entries.has("yarn.lock")) return (script) => ({ command: "yarn", args: ["run", script] });
	return (script) => ({ command: "npm", args: ["run", script] });
}

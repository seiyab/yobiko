import { Provider, Task, Workspace } from "#app/core/provider.js";
import { promises as fs, PathLike } from "node:fs";
import path from "node:path";
import { parse, TomlValue } from "smol-toml";

export const uv: Provider = { name: "uv", workspace };

async function workspace(p: PathLike): Promise<Workspace | null> {
	const configPath = path.join(p.toString(), "pyproject.toml");
	try {
		await fs.access(configPath);
	} catch {
		return null;
	}

	const config = parse(await fs.readFile(configPath, "utf8"));
	if (!isTable(config.project) || !isTable(config.project.scripts)) return null;

	const tasks = Object.entries(config.project.scripts)
		.filter((entry): entry is [string, string] => typeof entry[1] === "string")
		.map(
			([name, entryPoint]): Task => ({
				name,
				cwd: p.toString(),
				command: "uv",
				args: ["run", name],
				content: entryPoint,
			}),
		);

	return {
		dir: p.toString(),
		provider: uv,
		tasks,
	};
}

function isTable(value: TomlValue | undefined): value is Record<string, TomlValue> {
	return (
		value != null && typeof value === "object" && !Array.isArray(value) && !(value instanceof Date)
	);
}

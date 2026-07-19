import { Provider, Task, Workspace } from "#app/core/provider.js";
import { promises as fs, PathLike } from "node:fs";
import path from "node:path";
import { parse, TomlValue } from "smol-toml";

export const mise: Provider = { name: "mise", workspace };

async function workspace(p: PathLike): Promise<Workspace | null> {
	const configPath = path.join(p.toString(), "mise.toml");
	try {
		await fs.access(configPath);
	} catch {
		return null;
	}

	const config = parse(await fs.readFile(configPath, "utf8"));
	const tasks = config.tasks;
	if (!isTable(tasks)) return null;

	return {
		dir: p.toString(),
		provider: mise,
		tasks: Object.entries(tasks).map(
			([name, definition]): Task => ({
				name,
				cwd: p.toString(),
				command: "mise",
				args: ["run", name],
				content: taskContent(definition),
			}),
		),
	};
}

function taskContent(definition: TomlValue): string | undefined {
	if (typeof definition === "string") return definition;
	if (Array.isArray(definition))
		return definition.filter((value) => typeof value === "string").join("\n");
	if (!isTable(definition)) return undefined;

	const run = definition.run;
	if (typeof run === "string") return run;
	if (Array.isArray(run)) return run.filter((value) => typeof value === "string").join("\n");
	if (typeof definition.description === "string") return definition.description;
	return undefined;
}

function isTable(value: TomlValue | undefined): value is Record<string, TomlValue> {
	return (
		value != null && typeof value === "object" && !Array.isArray(value) && !(value instanceof Date)
	);
}

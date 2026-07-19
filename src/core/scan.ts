import { Dirent, PathLike } from "node:fs";
import { Provider, Workspace } from "./provider.js";
import { promises as fs } from "node:fs";
import { join, relative } from "node:path";
import ignore, { Ignore } from "ignore";

const ignoredDirectories = new Set([".git", "node_modules"]);

type Scanner = {
	scan: (dir: PathLike, opts: ScanOptions) => Promise<Workspace[]>;
};

export function setup(providers: Provider[]): Scanner {
	return { scan };

	async function scan(path: PathLike, opts: ScanOptions): Promise<Workspace[]> {
		return scanDir(path.toString(), opts.depth, []);
	}

	async function scanDir(
		path: string,
		depth: number,
		ignores: IgnoreContext[],
	): Promise<Workspace[]> {
		if (depth <= 0) return [];

		const ws: Workspace[] = [];
		for (const provider of providers) {
			try {
				const w = await provider.workspace(path);
				if (w == null) continue;
				ws.push(w);
			} catch {
				// skip
			}
		}

		const nestedIgnores = await addGitignore(path, ignores);
		using dir = await fs.opendir(path);
		let e: Dirent | null;
		while ((e = await dir.read())) {
			if (!e.isDirectory()) continue;
			if (ignoredDirectories.has(e.name)) continue;

			const p = join(path, e.name);
			if (isIgnored(p, nestedIgnores)) continue;
			ws.push(...(await scanDir(p, depth - 1, nestedIgnores)));
		}

		return ws;
	}
}

type IgnoreContext = {
	base: string;
	matcher: Ignore;
};

async function addGitignore(path: string, contexts: IgnoreContext[]): Promise<IgnoreContext[]> {
	try {
		const patterns = await fs.readFile(join(path, ".gitignore"), "utf8");
		return [...contexts, { base: path, matcher: ignore().add(patterns) }];
	} catch {
		return contexts;
	}
}

function isIgnored(path: string, contexts: IgnoreContext[]): boolean {
	let ignored = false;
	for (const context of contexts) {
		const result = context.matcher.test(`${relative(context.base, path)}/`);
		if (result.ignored) ignored = true;
		if (result.unignored) ignored = false;
	}
	return ignored;
}

type ScanOptions = {
	depth: number;
};

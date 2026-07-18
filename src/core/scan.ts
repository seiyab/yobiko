import { Dirent, PathLike } from "node:fs";
import { Provider, Workspace } from "./provider.js";
import { promises as fs } from "node:fs";
import { join } from "node:path";

type Scanner = {
	scan: (dir: PathLike, opts: ScanOptions) => Promise<Workspace[]>;
};

export function setup(providers: Provider[]): Scanner {
	return { scan };

	async function scan(path: PathLike, opts: ScanOptions): Promise<Workspace[]> {
		const { depth } = opts;
		if (depth <= 0) return [];

		const ws: Workspace[] = [];
		for (const provider of providers) {
			try {
				const w = await provider.workspace(path);
				if (w == null) continue;
				ws.push(w);
			} catch (_) {
				// skip
			}
		}

		using dir = await fs.opendir(path);
		let e: Dirent | null;
		while ((e = await dir.read())) {
			if (!e.isDirectory()) continue;
			const p = join(path.toString(), e.name);
			ws.push(...(await scan(p, { depth: depth - 1 })));
		}

		return ws;
	}
}

type ScanOptions = {
	depth: number;
};

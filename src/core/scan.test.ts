import { promises as fs } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, test } from "vitest";
import { Provider } from "./provider.js";
import { setup } from "./scan.js";

const roots: string[] = [];
afterEach(async () => {
	await Promise.all(roots.splice(0).map((root) => fs.rm(root, { recursive: true, force: true })));
});

describe("scan", () => {
	test("prunes ignored directories and supports negation", async () => {
		const root = await fixture({
			".gitignore": "ignored/*\n!ignored/kept/\n",
			"ignored/dropped/workspace": "",
			"ignored/kept/workspace": "",
			"visible/workspace": "",
		});
		expect(await scan(root)).toEqual([join(root, "ignored", "kept"), join(root, "visible")]);
	});

	test("applies nested gitignore files relative to their directory", async () => {
		const root = await fixture({
			"one/.gitignore": "/ignored/\n",
			"one/ignored/workspace": "",
			"one/visible/workspace": "",
			"two/ignored/workspace": "",
		});
		expect(await scan(root)).toEqual([join(root, "one", "visible"), join(root, "two", "ignored")]);
	});

	test("always skips .git", async () => {
		const root = await fixture({ ".git/workspace": "", "visible/workspace": "" });
		expect(await scan(root)).toEqual([join(root, "visible")]);
	});

	test("always skips node_modules", async () => {
		const root = await fixture({
			".gitignore": "!node_modules/\n",
			"node_modules/package/workspace": "",
			"visible/workspace": "",
		});
		expect(await scan(root)).toEqual([join(root, "visible")]);
	});
});

async function scan(root: string): Promise<string[]> {
	const provider: Provider = {
		name: "test",
		async workspace(path) {
			const dir = path.toString();
			try {
				await fs.access(join(dir, "workspace"));
				return { dir, provider, tasks: [] };
			} catch {
				return null;
			}
		},
	};
	return (await setup([provider]).scan(root, { depth: 10 })).map(({ dir }) => dir).sort();
}

async function fixture(files: Record<string, string>): Promise<string> {
	const root = await fs.mkdtemp(join(tmpdir(), "yobiko-scan-"));
	roots.push(root);
	for (const [path, contents] of Object.entries(files)) {
		const target = join(root, path);
		await fs.mkdir(join(target, ".."), { recursive: true });
		await fs.writeFile(target, contents);
	}
	return root;
}

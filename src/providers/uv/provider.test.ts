import { promises as fs } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, describe, expect, test } from "vitest";
import { uv } from "./provider.js";

const directories: string[] = [];

afterEach(async () => {
	await Promise.all(directories.splice(0).map((dir) => fs.rm(dir, { recursive: true })));
});

describe("uv provider", () => {
	test("returns null without a pyproject.toml", async () => {
		const dir = await temporaryDirectory();

		await expect(uv.workspace(dir)).resolves.toBeNull();
	});

	test("returns null when the project has no scripts", async () => {
		const dir = await temporaryDirectory();
		await fs.writeFile(
			path.join(dir, "pyproject.toml"),
			["[project]", 'name = "example"', 'version = "0.1.0"'].join("\n"),
		);

		await expect(uv.workspace(dir)).resolves.toBeNull();
	});

	test("reads project scripts", async () => {
		const dir = await temporaryDirectory();
		await fs.writeFile(
			path.join(dir, "pyproject.toml"),
			[
				"[project]",
				'name = "example"',
				'version = "0.1.0"',
				"",
				"[project.scripts]",
				'example = "example:main"',
				'serve = "example.server:run"',
				"",
				"[project.gui-scripts]",
				'example-gui = "example.gui:main"',
			].join("\n"),
		);

		await expect(uv.workspace(dir)).resolves.toEqual({
			dir,
			provider: uv,
			tasks: [
				{
					name: "example",
					cwd: dir,
					command: "uv",
					args: ["run", "example"],
					content: "example:main",
				},
				{
					name: "serve",
					cwd: dir,
					command: "uv",
					args: ["run", "serve"],
					content: "example.server:run",
				},
			],
		});
	});
});

async function temporaryDirectory(): Promise<string> {
	const dir = await fs.mkdtemp(path.join(os.tmpdir(), "yobiko-uv-"));
	directories.push(dir);
	return dir;
}

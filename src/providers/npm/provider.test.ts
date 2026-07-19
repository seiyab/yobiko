import { promises as fs } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, describe, expect, test } from "vitest";
import { npm } from "./provider.js";

const directories: string[] = [];

afterEach(async () => {
	await Promise.all(directories.splice(0).map((dir) => fs.rm(dir, { recursive: true })));
});

describe("npm provider", () => {
	test("returns null without a package.json", async () => {
		const dir = await temporaryDirectory();

		await expect(npm.workspace(dir)).resolves.toBeNull();
	});

	test.each([
		{ lockfile: undefined, command: "npm" },
		{ lockfile: "yarn.lock", command: "yarn" },
		{ lockfile: "pnpm-lock.yaml", command: "pnpm" },
	])("reads scripts and uses $command", async ({ lockfile, command }) => {
		const dir = await temporaryDirectory();
		await fs.writeFile(
			path.join(dir, "package.json"),
			JSON.stringify({
				scripts: {
					build: "tsc",
					test: "vitest --run",
				},
			}),
		);
		if (lockfile != null) await fs.writeFile(path.join(dir, lockfile), "");

		await expect(npm.workspace(dir)).resolves.toEqual({
			dir,
			provider: npm,
			tasks: [
				{
					name: "build",
					cwd: dir,
					command,
					args: ["run", "build"],
					content: "tsc",
				},
				{
					name: "test",
					cwd: dir,
					command,
					args: ["run", "test"],
					content: "vitest --run",
				},
			],
		});
	});
});

async function temporaryDirectory(): Promise<string> {
	const dir = await fs.mkdtemp(path.join(os.tmpdir(), "yobiko-npm-"));
	directories.push(dir);
	return dir;
}

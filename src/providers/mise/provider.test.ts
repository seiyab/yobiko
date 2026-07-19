import { promises as fs } from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, describe, expect, test } from "vitest";
import { mise } from "./provider.js";

const directories: string[] = [];

afterEach(async () => {
	await Promise.all(directories.splice(0).map((dir) => fs.rm(dir, { recursive: true })));
});

describe("mise provider", () => {
	test("returns null without a mise.toml", async () => {
		const dir = await temporaryDirectory();

		await expect(mise.workspace(dir)).resolves.toBeNull();
	});

	test("reads shorthand and detailed tasks", async () => {
		const dir = await temporaryDirectory();
		await fs.writeFile(
			path.join(dir, "mise.toml"),
			[
				"[tasks]",
				'build = "cargo build"',
				'test = ["cargo test", "./scripts/test-e2e.sh"]',
				"",
				"[tasks.lint]",
				'description = "Lint the project"',
				'run = "cargo clippy"',
				"",
				"[tasks.ci]",
				'description = "Run CI tasks"',
				'depends = ["build", "lint", "test"]',
			].join("\n"),
		);

		const workspace = await mise.workspace(dir);

		expect(workspace).toEqual({
			dir,
			provider: mise,
			tasks: [
				{
					name: "build",
					cwd: dir,
					command: "mise",
					args: ["run", "build"],
					content: "cargo build",
				},
				{
					name: "test",
					cwd: dir,
					command: "mise",
					args: ["run", "test"],
					content: "cargo test\n./scripts/test-e2e.sh",
				},
				{
					name: "lint",
					cwd: dir,
					command: "mise",
					args: ["run", "lint"],
					content: "cargo clippy",
				},
				{
					name: "ci",
					cwd: dir,
					command: "mise",
					args: ["run", "ci"],
					content: "Run CI tasks",
				},
			],
		});
	});
});

async function temporaryDirectory(): Promise<string> {
	const dir = await fs.mkdtemp(path.join(os.tmpdir(), "yobiko-mise-"));
	directories.push(dir);
	return dir;
}

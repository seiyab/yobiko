import { Text } from "ink";
import { render } from "ink-testing-library";
import { describe, expect, test, vi } from "vitest";
import { Select } from "./select.js";

describe("<Select />", () => {
	test("can move cursor", async () => {
		const opts = options(5);
		const onSelect = vi.fn<() => void>();
		const onCursor = vi.fn<() => void>();
		const { lastFrame, stdin } = render(
			<Select items={opts} active onSelect={onSelect} onCursor={onCursor} height={6} />,
		);
		expect(lastFrame()).toEqual(
			[
				"> 0", //
				"  1",
				"  2",
				"  3",
				"  4",
				"",
			].join("\n"),
		);
		expect(onCursor).toHaveBeenLastCalledWith(0);

		stdin.write("jj");

		await vi.waitFor(() => {
			expect(lastFrame()).toEqual(
				[
					"  0", //
					"  1",
					"> 2",
					"  3",
					"  4",
					"",
				].join("\n"),
			);
		});
		expect(onCursor).toHaveBeenLastCalledWith(2);
	});

	test("scrolls down", async () => {
		const opts = options(10);
		const onSelect = vi.fn<() => void>();
		const onCursor = vi.fn<() => void>();
		const { lastFrame, stdin } = render(
			<Select items={opts} active onSelect={onSelect} onCursor={onCursor} height={7} />,
		);

		stdin.write("j".repeat(6));
		await vi.waitFor(() => {
			expect(lastFrame()).toEqual(
				[
					"  2", //
					"  3",
					"  4",
					"  5",
					"> 6",
					"  7",
					"  8",
				].join("\n"),
			);
		});

		stdin.write("j".repeat(3));
		await vi.waitFor(() => {
			expect(lastFrame()).toEqual(
				[
					"  3", //
					"  4",
					"  5",
					"  6",
					"  7",
					"  8",
					"> 9",
				].join("\n"),
			);
		});
	});

	test("scrolls up", async () => {
		const opts = options(10);
		const onSelect = vi.fn<() => void>();
		const onCursor = vi.fn<() => void>();
		const { lastFrame, stdin } = render(
			<Select items={opts} active onSelect={onSelect} onCursor={onCursor} height={7} />,
		);

		stdin.write("j".repeat(6));
		await vi.waitFor(() => {
			expect(lastFrame()).toEqual(
				[
					"  2", //
					"  3",
					"  4",
					"  5",
					"> 6",
					"  7",
					"  8",
				].join("\n"),
			);
		});

		stdin.write("k".repeat(3));
		await vi.waitFor(() => {
			expect(lastFrame()).toEqual(
				[
					"  1", //
					"  2",
					"> 3",
					"  4",
					"  5",
					"  6",
					"  7",
				].join("\n"),
			);
		});
	});
});

function options(size: number) {
	return Array.from({ length: size }).map((_, i) => ({
		value: i,
		display: <Text>{`${i}`}</Text>,
	}));
}

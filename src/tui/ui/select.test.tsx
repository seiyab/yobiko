import { Text } from "ink";
import { render } from "ink-testing-library";
import { describe, expect, test, vi } from "vitest";
import { Select } from "./select.js";

describe("<Select />", () => {
	test("can move cursor", async () => {
		const opts = options(5);
		const onSelect = vi.fn();
		const onCursor = vi.fn();
		const { lastFrame, stdin, rerender } = render(
			<Select
				items={opts}
				active
				onSelect={onSelect}
				onCursor={onCursor}
				height={6}
			/>,
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
});

function options(size: number) {
	return Array.from({ length: size }).map((_, i) => ({
		value: i,
		display: <Text>{`${i}`}</Text>,
	}));
}

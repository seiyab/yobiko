import { useEffect, useMemo, useRef, useState } from "react";
import { BoxAttributes } from "./box-attributes.js";
import { Box, Text, useBoxMetrics } from "ink";
import { useKeyMap } from "../input.js";

type Item<T> = {
	value: T;
	display: React.ReactNode;
};

type Props<T> = {
	items: Item<T>[];
	active: boolean;
	selectDescription?: string;
	onCursor?: (value: T | null) => void;
	onSelect?: (value: T) => void;
} & BoxAttributes;

export function Select<T>({
	items,
	active,
	selectDescription,
	onCursor,
	onSelect,
	...rest
}: Props<T>) {
	const [needle, setNeedle] = useState(0);
	const itemUnderCursor = items.at(needle);
	useEffect(() => {
		onCursor?.(itemUnderCursor?.value ?? null);
	}, [itemUnderCursor?.value]);
	const validNeedle = clamp(needle, { min: 0, max: items.length - 1 });
	if (needle !== validNeedle) {
		setNeedle(validNeedle);
	}

	useKeyMap(
		useMemo(() => {
			if (!active) return {};
			const range = { min: 0, max: items.length - 1 };
			return {
				k: {
					action: () => setNeedle((prev) => clamp(prev - 1, range)),
					description: "move cursor up",
				},
				j: {
					action: () => setNeedle((prev) => clamp(prev + 1, range)),
					description: "move cursor down",
				},
			};
		}, [items.length, active]),
	);

	useKeyMap(
		useMemo(() => {
			if (!active) return {};
			return {
				"<return>": {
					action: () => {
						if (itemUnderCursor == null) return;
						onSelect?.(itemUnderCursor.value);
					},
					description: selectDescription ?? "select item under cursor",
				},
			};
		}, [itemUnderCursor, selectDescription]),
	);

	const ref = useRef(null);
	const mx = useBoxMetrics(ref);

	const [scroll, setScroll] = useState(0);
	const height = mx.hasMeasured ? mx.height : 0;
	const maxScroll = Math.max(0, items.length - height);
	if (scroll < 0 || maxScroll < scroll) {
		setScroll(clamp(scroll, { min: 0, max: maxScroll }));
	}
	useEffect(() => {
		const position = needle - scroll;
		const maxRoom = Math.min(2, Math.floor((height - 1) / 2));
		const roomTop = Math.min(maxRoom, needle);
		const roomBottom = Math.min(maxRoom, items.length - 1 - needle);
		const validPosition = clamp(position, {
			min: roomTop,
			max: height - roomBottom - 1,
		});
		if (position !== validPosition) {
			setScroll(scroll - validPosition + position);
		}
	}, [needle, scroll, items.length, height]);

	return (
		<Box ref={ref} overflow="hidden" {...rest}>
			<Box flexDirection="column" position="absolute" top={-scroll}>
				{items.map((item, i) => (
					<Box key={JSON.stringify(item.value)} flexDirection="row" height={1} overflow="hidden">
						<Text>{i === needle ? "> " : "  "}</Text>
						<Box>{item.display}</Box>
					</Box>
				))}
			</Box>
		</Box>
	);
}
Select satisfies React.FC<Props<never>>;

function clamp(n: number, { min, max }: { min: number; max: number }): number {
	return Math.max(min, Math.min(n, max));
}

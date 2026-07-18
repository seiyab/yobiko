import { Box, Text, useInput } from "ink";
import { useEffect, useRef, useState } from "react";
import TextInput from "ink-text-input";
import { Task, Workspace } from "#app/core/provider.js";
import { Pane } from "#app/tui/ui/pane.js";
import { BoxAttributes } from "../ui/box-attributes.js";
import { runner } from "#app/core/runner.js";

type Props = {
	workspaces: Workspace[];
	onHoverTask: (task: Task | null) => void;
} & BoxAttributes;

export function SelectTask({ workspaces, onHoverTask, ...rest }: Props) {
	const [query, setQuery] = useState("");
	const list = workspaces.flatMap((w) => {
		return w.tasks.filter(
			(t) => t.name.includes(query) || t.cwd.includes(query) || t.command.includes(query),
		);
	});
	const [cursor, setCursor] = useState(0);
	const [mode, setMode] = useState<"normal" | "insert">("normal");
	if (list.length > 0 && cursor >= list.length) {
		setCursor(list.length - 1);
	}
	const ref = useRef(null);
	// const met = useBoxMetrics(ref);
	useInput((input, key) => {
		if (mode === "insert") {
			if (key.escape) setMode("normal");
			if (key.ctrl && input === "[") setMode("normal");
			return;
		}
		if (key.return) {
			const task = list.at(cursor);
			if (task != null) {
				runner.spawn(task);
			}
			return;
		}
		switch (input) {
			case "i":
			case "/":
				setMode("insert");
				break;
			case "j":
				moveCursor((prev) => prev + 1);
				break;
			case "k":
				moveCursor((prev) => prev - 1);
				break;
			default:
			// nothing
		}
	});
	const taskUnderCursor = list.at(cursor) ?? null;
	useEffect(() => {
		onHoverTask(taskUnderCursor);
	}, [taskUnderCursor]);

	return (
		<Pane name="Select Task" flexDirection="column" {...rest}>
			<Box flexDirection="column">
				<Box flexDirection="row" borderBottom borderColor="gray">
					<Text>Filter[/]: </Text>
					<TextInput
						value={query}
						focus={mode === "insert"}
						onChange={setQuery}
						onSubmit={() => setMode("normal")}
					/>
				</Box>
				<Box ref={ref} flexGrow={1} flexDirection="column" overflow="hidden">
					{list.map((task, i) => (
						<Box key={JSON.stringify([task.cwd, task.command, task.args])} flexDirection="row">
							<Text>{i === cursor ? "> " : "  "}</Text>
							<Text>{`(${task.cwd}) `}</Text>
							<Text>{task.command}</Text>
							<Text> {task.args?.join(" ")}</Text>
						</Box>
					))}
				</Box>
			</Box>
		</Pane>
	);

	function moveCursor(action: (prev: number) => number): void {
		setCursor((prev) => clamp(action(prev), { min: 0, max: list.length - 1 }));
		onHoverTask(list[action(cursor)] ?? null);
	}
}
SelectTask satisfies React.FC<Props>;

function clamp(n: number, { min, max }: { min: number; max: number }): number {
	return Math.max(min, Math.min(n, max));
}

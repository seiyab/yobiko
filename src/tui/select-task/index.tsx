import { Box, Text, useInput } from "ink";
import { useRef, useState } from "react";
import TextInput from "ink-text-input";
import { Workspace } from "#app/core/provider.js";
import { Pane } from "#app/tui/ui/pane.js";
import { BoxAttributes } from "../ui/box-attributes.js";
import { runner } from "#app/core/runner.js";

type Props = {
	workspaces: Workspace[];
} & BoxAttributes;

export function SelectTask({ workspaces, ...rest }: Props) {
	const [query, setQuery] = useState("");
	const list = workspaces.flatMap((w) => {
		return w.tasks.filter(
			(t) => t.name.includes(query) || t.cwd.includes(query) || t.command.includes(query),
		);
	});
	const [cursor, setCursor] = useState(0);
	const [mode, setMode] = useState<"normal" | "insert">("normal");
	if (list.length > 1 && cursor >= list.length) {
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
				setCursor((prev) => Math.min(prev + 1, list.length - 1));
				break;
			case "k":
				setCursor((prev) => Math.max(prev - 1, 0));
				break;
			default:
			// nothing
		}
	});

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
}
SelectTask satisfies React.FC<Props>;

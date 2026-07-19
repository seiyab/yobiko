import { Box, Text, useInput } from "ink";
import { useState } from "react";
import TextInput from "ink-text-input";
import { Task, Workspace } from "#app/core/provider.js";
import { Pane } from "#app/tui/ui/pane.js";
import { BoxAttributes } from "../ui/box-attributes.js";
import { runner } from "#app/core/runner.js";
import { Select } from "../ui/select.js";

type Props = {
	workspaces: Workspace[];
	onHoverTask: (task: Task | null) => void;
} & BoxAttributes;

export function SelectTask({ workspaces, onHoverTask, ...rest }: Props) {
	const [query, setQuery] = useState("");
	const list = workspaces.flatMap((w) => {
		return w.tasks.filter(
			(t) =>
				t.name.includes(query) ||
				t.cwd.includes(query) ||
				t.command.includes(query),
		);
	});
	const [mode, setMode] = useState<"normal" | "insert">("normal");
	useInput((input, key) => {
		if (mode === "insert") {
			if (key.escape) setMode("normal");
			if (key.ctrl && input === "[") setMode("normal");
			return;
		}
		switch (input) {
			case "i":
			case "/":
				setMode("insert");
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
				<Select
					items={list.map((task) => ({
						value: task,
						display: (
							<Box flexDirection="row">
								<Text>{`(${task.cwd}) `}</Text>
								<Text>{task.command}</Text>
								<Text> {task.args?.join(" ")}</Text>
							</Box>
						),
					}))}
					active={mode === "normal"}
					onCursor={onHoverTask}
					onSelect={(task) => runner.spawn(task)}
				/>
				{/*<Box ref={ref} flexGrow={1} flexDirection="column" overflow="hidden">
					{list.map((task, i) => (
						<Box key={JSON.stringify([task.cwd, task.command, task.args])} flexDirection="row">
							<Text>{i === cursor ? "> " : "  "}</Text>
							<Text>{`(${task.cwd}) `}</Text>
							<Text>{task.command}</Text>
							<Text> {task.args?.join(" ")}</Text>
						</Box>
					))}
				</Box>*/}
			</Box>
		</Pane>
	);
}
SelectTask satisfies React.FC<Props>;

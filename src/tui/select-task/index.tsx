import { Box, Text, useInput } from "ink";
import { useMemo, useState } from "react";
import TextInput from "ink-text-input";
import { Task, Workspace } from "#app/core/provider.js";
import { Pane } from "#app/tui/ui/pane.js";
import { BoxAttributes } from "../ui/box-attributes.js";
import { runner } from "#app/core/runner.js";
import { Select } from "../ui/select.js";
import { useFocus, useKeyMap } from "../input.js";

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
	const queryFocus = useFocus();
	useInput((input, key) => {
		if (!queryFocus.active) return;
		if (key.escape) queryFocus.release();
		if (key.ctrl && input === "[") queryFocus.release();
	});
	useKeyMap(
		useMemo(
			() => ({
				"/": { action: queryFocus.capture, description: "input search query" },
			}),
			[queryFocus.capture],
		),
	);

	return (
		<Pane name="Select Task" flexDirection="column" flexGrow={1} {...rest}>
			<Box flexDirection="column" flexGrow={1}>
				<Box flexDirection="row" borderBottom borderColor="gray">
					<Text>Filter[/]: </Text>
					<TextInput
						value={query}
						focus={queryFocus.active}
						onChange={setQuery}
						onSubmit={queryFocus.release}
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
					active={!queryFocus.active}
					onCursor={onHoverTask}
					onSelect={(task) => runner.spawn(task)}
					flexGrow={1}
				/>
			</Box>
		</Pane>
	);
}
SelectTask satisfies React.FC<Props>;

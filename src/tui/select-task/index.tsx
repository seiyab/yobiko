import { Box, Text } from "ink";
import { forwardRef, useImperativeHandle, useState } from "react";
import TextInput from "ink-text-input";
import { Task, Workspace } from "#app/core/provider.js";
import { Pane } from "#app/tui/ui/pane.js";
import { BoxAttributes } from "../ui/box-attributes.js";
import { Select } from "../ui/select.js";
import { useFocus, useKeyMap } from "../input.js";

type Props = {
	workspaces: Workspace[];
	onHoverTask: (task: Task | null) => void;
} & BoxAttributes;

type Handle = {
	focusQuery: () => void;
};
export type SelectTaskHandle = Handle;

export const SelectTask = forwardRef<Handle, Props>(({ workspaces, onHoverTask, ...rest }, ref) => {
	const [query, setQuery] = useState("");
	const list = workspaces.flatMap((w) => {
		return w.tasks.filter(
			(t) => t.name.includes(query) || t.cwd.includes(query) || t.command.includes(query),
		);
	});
	const queryFocus = useFocus();
	useImperativeHandle<Handle, Handle>(ref, () => ({ focusQuery: queryFocus.capture }), [
		queryFocus.capture,
	]);
	useKeyMap({
		"<esc>": {
			action: queryFocus.release,
			description: "blur from search query",
			focus: queryFocus.id,
		},
		"<c-[>": {
			action: queryFocus.release,
			description: "blur from search query",
			focus: queryFocus.id,
		},
	});
	useKeyMap({
		"/": { action: queryFocus.capture, description: "input search query" },
		"<c-u>": {
			action: () => setQuery(""),
			description: "clear search query",
			focus: queryFocus.id,
		},
	});

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
					flexGrow={1}
				/>
			</Box>
		</Pane>
	);
});

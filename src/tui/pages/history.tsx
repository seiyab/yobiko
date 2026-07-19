import { runner } from "#app/core/runner.js";
import { Box, Spacer, Text } from "ink";
import { useState, useSyncExternalStore } from "react";
import { Pane } from "../ui/pane.js";
import { Select } from "../ui/select.js";
import { TaskStatusIndicator } from "../task-status-indicator.js";

export function History() {
	const runs = useSyncExternalStore(runner.state.subscribe, runner.state.getSnapshot);
	const [needle, setNeedle] = useState<string | null>(null);
	return (
		<Box flexDirection="row" flexGrow={1}>
			<Pane name="Runs" flexGrow={2} flexBasis={0}>
				<Select
					active
					items={runs.toReversed().map((run) => ({
						value: run.id,
						display: (
							<Box flexDirection="row">
								<TaskStatusIndicator status={run.status} />
								<Text color="gray">({run.task.cwd})</Text>
								<Text>
									{run.task.command} ${run.task.args?.join(" ")}
								</Text>
								<Spacer />
								<Text color="gray">[{run.createdAt.toISOString()}]</Text>
							</Box>
						),
					}))}
					onCursor={setNeedle}
				/>
			</Pane>
			<Pane name="Output" flexGrow={5} flexBasis={0}>
				<Text>{needle}</Text>
			</Pane>
		</Box>
	);
}

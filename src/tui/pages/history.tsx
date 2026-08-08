import { runner, RunOutput } from "#app/core/runner.js";
import { Box, Text } from "ink";
import { useState, useSyncExternalStore } from "react";
import { Pane } from "../ui/pane.js";
import { Select } from "../ui/select.js";
import { TaskStatusIndicator } from "../task-status-indicator.js";
import { useKeyMap } from "../input.js";

export function History() {
	const runs = useSyncExternalStore(runner.state.subscribe, runner.state.getSnapshot);
	const [selectedRunId, setSelectedRunId] = useState<string | null>(null);
	const selectedRun = runs.find((run) => run.id === selectedRunId);
	useKeyMap({
		r: {
			action: () => {
				const run = runs.find((r) => r.id === selectedRunId);
				if (run == null) return;
				void runner.spawn(run.task);
			},
			description: "rerun the task",
		},
		x: {
			action: () => {
				const run = runs.find((r) => r.id === selectedRunId);
				if (run == null) return;
				void run.kill();
			},
			description: "kill the task",
		},
	});
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
							</Box>
						),
					}))}
					onCursor={setSelectedRunId}
				/>
			</Pane>
			<Pane name="Output" flexGrow={5} flexBasis={0}>
				<Output output={selectedRun?.output} />
			</Pane>
		</Box>
	);
}

function Output({ output }: { output?: RunOutput }) {
	if (output == null) return <Text />;
	return <RunOutputText output={output} />;
}

function RunOutputText({ output }: { output: RunOutput }) {
	const text = useSyncExternalStore(output.subscribe, output.getSnapshot);
	return <Text>{text}</Text>;
}

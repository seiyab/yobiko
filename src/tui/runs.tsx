import { Box, Text } from "ink";
import { BoxAttributes } from "./ui/box-attributes.js";
import { Pane } from "./ui/pane.js";
import { useSyncExternalStore } from "react";
import { runner } from "#app/core/runner.js";
import { TaskStatusIndicator } from "./task-status-indicator.js";

type Props = BoxAttributes;

export function Runs({ ...rest }: Props) {
	const runs = useSyncExternalStore(runner.state.subscribe, runner.state.getSnapshot);
	return (
		<Pane name="Runs" {...rest}>
			<Box flexDirection="column">
				{runs.toReversed().map((run) => (
					<Box key={run.id} flexDirection="row" gap={1}>
						<TaskStatusIndicator status={run.status} />
						<Text>
							{`(${run.task.cwd}) ${run.task.command}`} ${run.task.args?.join(" ")}
						</Text>
					</Box>
				))}
			</Box>
		</Pane>
	);
}

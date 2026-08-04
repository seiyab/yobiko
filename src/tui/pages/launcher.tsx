import { Task, Workspace } from "#app/core/provider.js";
import { Box } from "ink";
import { useState } from "react";
import { SelectTask } from "#app/tui/select-task/index.js";
import { Runs } from "#app/tui/runs.js";
import { TaskDetail } from "#app/tui/task-detail.js";
import { runner } from "#app/core/runner.js";
import { useKeyMap } from "../input.js";

type Props = {
	workspaces: Workspace[];
};

export function Launcher({ workspaces }: Props) {
	const [task, setTask] = useState<Task | null>(null);
	useKeyMap(
		task == null
			? {}
			: {
					"<return>": {
						action: () => runner.spawn(task),
						description: "launch task under the cursor",
					},
				},
	);
	return (
		<Box flexDirection="column" flexGrow={1}>
			<SelectTask flexGrow={5} flexBasis={0} workspaces={workspaces} onHoverTask={setTask} />
			<Box flexGrow={2} flexBasis={0}>
				<Runs flexGrow={1} flexBasis={0} />
				<TaskDetail flexGrow={1} flexBasis={0} task={task} />
			</Box>
		</Box>
	);
}
Launcher satisfies React.FC<Props>;

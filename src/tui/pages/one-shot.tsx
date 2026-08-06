import { Task, Workspace } from "#app/core/provider.js";
import { Box, useApp } from "ink";
import { useState } from "react";
import { SelectTask } from "#app/tui/select-task/index.js";
import { TaskDetail } from "#app/tui/task-detail.js";
import { useKeyMap } from "../input.js";
import { register } from "#app/core/one-shot-runner.js";

type Props = {
	workspaces: Workspace[];
};

export function OneShot({ workspaces }: Props) {
	const { exit } = useApp();
	const [task, setTask] = useState<Task | null>(null);
	useKeyMap(
		task == null
			? {}
			: {
					"<return>": {
						action: () => {
							register(task);
							exit();
						},
						description: "launch task under the cursor",
					},
				},
	);
	return (
		<Box flexDirection="column" flexGrow={1}>
			<SelectTask flexGrow={5} flexBasis={0} workspaces={workspaces} onHoverTask={setTask} />
			<Box flexGrow={2} flexBasis={0}>
				<TaskDetail flexGrow={1} flexBasis={0} task={task} />
			</Box>
		</Box>
	);
}
OneShot satisfies React.FC<Props>;

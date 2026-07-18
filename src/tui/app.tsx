import React, { useState } from "react";
import { useApp, Box, Text, useInput } from "ink";
import { useQuery } from "@tanstack/react-query";
import { ecq } from "@seiyab/ecq";
import { setup } from "#app/core/scan.js";
import { npm } from "#app/providers/npm/provider.js";
import Spinner from "ink-spinner";
import { SelectTask } from "./select-task/index.js";
import { Runs } from "./runs.js";
import { TaskDetail } from "./task-detail.js";
import { Task } from "#app/core/provider.js";

const s = ecq.client(setup([npm]));

export function App() {
	const ws = useQuery(s.scan("./", { depth: 3 }));
	const { exit } = useApp();
	const [task, setTask] = useState<Task | null>(null);
	useInput((input) => {
		if (input === "q") {
			exit();
		}
	});
	return (
		<Box flexDirection="column" alignItems="stretch" width="100%" height="100%">
			{ws.isPending ? (
				<Box>
					<Spinner type="bouncingBar" />
					<Text>loading project...</Text>
				</Box>
			) : ws.isError ? (
				<Text>error</Text>
			) : (
				<Box flexDirection="column" flexGrow={1}>
					<SelectTask flexGrow={5} flexBasis={0} workspaces={ws.data} onHoverTask={setTask} />
					<Box flexGrow={2} flexBasis={0}>
						<Runs flexGrow={1} flexBasis={0} />
						<TaskDetail flexGrow={1} flexBasis={0} task={task} />
					</Box>
				</Box>
			)}
			<Box flexGrow={0} flexShrink={0}>
				<Text>Yobiko</Text>
			</Box>
		</Box>
	);
}

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
import { Launcher } from "./pages/launcher.js";

const s = ecq.client(setup([npm]));

export function App() {
	const ws = useQuery(s.scan("./", { depth: 3 }));
	const { exit } = useApp();
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
				<Launcher workspaces={ws.data} />
			)}
			<Box flexGrow={0} flexShrink={0}>
				<Text>Yobiko</Text>
			</Box>
		</Box>
	);
}

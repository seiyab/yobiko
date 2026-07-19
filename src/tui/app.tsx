import React, { useState } from "react";
import { useApp, Box, Text, useInput } from "ink";
import { useQuery } from "@tanstack/react-query";
import { ecq } from "@seiyab/ecq";
import { setup } from "#app/core/scan.js";
import { npm } from "#app/providers/npm/provider.js";
import { mise } from "#app/providers/mise/provider.js";
import { uv } from "#app/providers/uv/provider.js";
import Spinner from "ink-spinner";
import { SelectTask } from "./select-task/index.js";
import { Runs } from "./runs.js";
import { TaskDetail } from "./task-detail.js";
import { Task } from "#app/core/provider.js";
import { Launcher } from "./pages/launcher.js";
import { iife } from "#app/utils/iife.js";
import { History } from "./pages/history.js";

const s = ecq.client(setup([npm, mise, uv]));

export function App() {
	const ws = useQuery(s.scan("./", { depth: 3 }));
	const [tab, setTab] = useState<"launcher" | "project" | "history">("launcher");
	const { exit } = useApp();
	useInput((input) => {
		for (const c of input) {
			switch (c) {
				case "q":
					exit();
					break;
				case "L":
					setTab("launcher");
					break;
				case "H":
					setTab("history");
					break;
			}
		}
	});
	return (
		<Box flexDirection="column" alignItems="stretch" width="100%" height="100%">
			<Box flexDirection="row" gap={3}>
				<Text underline={tab == "launcher"}>[L]auncher</Text>
				<Text underline={tab == "project"}>[P]roject</Text>
				<Text underline={tab == "history"}>[H]istory</Text>
			</Box>
			{ws.isPending ? (
				<Box>
					<Spinner type="bouncingBar" />
					<Text>loading project...</Text>
				</Box>
			) : ws.isError ? (
				<Text>error</Text>
			) : (
				iife(() => {
					switch (tab) {
						case "launcher":
							return <Launcher workspaces={ws.data} />;
						case "project":
							return <Text>not implemented yet</Text>;
						case "history":
							return <History />;
						default:
							tab satisfies never;
					}
				})
			)}
			<Box flexGrow={0} flexShrink={0}>
				<Text>Yobiko</Text>
			</Box>
		</Box>
	);
}

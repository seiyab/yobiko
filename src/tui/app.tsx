import React from "react";
import { useApp, Box, Text, useInput } from "ink";
import { useQuery } from "@tanstack/react-query";
import { ecq } from "@seiyab/ecq";
import { promises as promisesFs } from "node:fs";
import { setup } from "#app/core/scan.js";
import { npm } from "#app/providers/npm/provider.js";
import Spinner from "ink-spinner";
import { SelectTask } from "./select-task/index.js";
import { Runs } from "./runs.js";

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
				<Box flexDirection="column" flexGrow={1}>
					<SelectTask flexGrow={5} workspaces={ws.data} />
					<Box flexGrow={2}>
						<Runs />
					</Box>
				</Box>
			)}
			<Box flexGrow={0} flexShrink={0}>
				<Text>Yobiko</Text>
			</Box>
		</Box>
	);
}

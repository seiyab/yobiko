import React from "react";
import { useApp, Box, Text, useInput } from "ink";
import { useQuery } from "@tanstack/react-query";
import { ecq } from "@seiyab/ecq";
import { promises as promisesFs } from "node:fs";
import { setup } from "#/core/project.js";
import { npm } from "#/providers/npm/provider.js";

const fs = ecq.client(promisesFs);
const s = ecq.client(setup([npm]));

export function App() {
	const ws = useQuery(s.scan("./", { depth: 3 }));
	const { exit } = useApp();
	useInput((input) => {
		if (input === "q") {
			exit();
		}
	});
	const p = useQuery(fs.readFile("./package.json", "utf8"));
	return (
		<Box flexDirection="column" alignItems="stretch" width="100%" height="100%">
			<Text>Hello</Text>
			<Text>{p.isPending ? "pending" : p.isSuccess ? p.data : "error"}</Text>
			{ws.isPending ? (
				<Text>pending</Text>
			) : ws.isError ? (
				<Text>error</Text>
			) : (
				ws.data.map((w) => <Text>{`${w.dir}:${w.provider.name}`}</Text>)
			)}
		</Box>
	);
}

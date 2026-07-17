import React from "react";
import { useApp, Box, Text, useInput } from "ink";
import { useQuery } from "@tanstack/react-query";
import { ecq } from "@seiyab/ecq";
import { promises as promisesFs } from "node:fs";

const fs = ecq.client(promisesFs);

export function App() {
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
		</Box>
	);
}

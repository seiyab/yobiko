import React from "react";
import { useApp, Box, Text, useInput } from "ink";

export function App() {
	const { exit } = useApp();
	useInput((input) => {
		if (input === "q") {
			exit();
		}
	});
	return (
		<Box flexDirection="column" alignItems="stretch" width="100%" height="100%">
			<Text>Hello</Text>
		</Box>
	);
}

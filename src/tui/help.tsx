import { Box, Text, useBoxMetrics } from "ink";
import { useHelp } from "./input.js";
import { useRef } from "react";
import { Pane } from "./ui/pane.js";

export function Help() {
	const help = useHelp();
	const keyLength = Math.max(...Object.keys(help).map((k) => k.length));

	return (
		<Box
			position="absolute"
			top={0}
			bottom={0}
			width="100%"
			height="100%"
			alignItems="center"
			justifyContent="center"
		>
			<Pane name="[?]Help" width="80%" height="80%">
				<Fill />
				<Box flexDirection="column">
					{Object.entries(help).map(([key, { description }]) => (
						<Box key={key}>
							<Box flexShrink={0}>
								<Text>{key}</Text>
								<Text>{" ".repeat(keyLength - key.length + 1)}: </Text>
							</Box>
							<Box flexShrink={0}>
								<Text>{description}</Text>
							</Box>
						</Box>
					))}
				</Box>
			</Pane>
		</Box>
	);
}

function Fill() {
	const ref = useRef(null);
	const m = useBoxMetrics(ref);

	return (
		<Box
			ref={ref}
			position="absolute"
			top={0}
			left={0}
			height="100%"
			width="100%"
			overflow="hidden"
			flexDirection="column"
		>
			<Text>
				{Array.from({ length: m.height })
					.map(() => " ".repeat(m.width))
					.join("\n")}
			</Text>
		</Box>
	);
}

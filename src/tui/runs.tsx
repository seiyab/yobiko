import { Box, Text } from "ink";
import { BoxAttributes } from "./ui/box-attributes.js";
import { Pane } from "./ui/pane.js";
import { useSyncExternalStore } from "react";
import { runner, RunStatus } from "#app/core/runner.js";
import Spinner from "ink-spinner";

type Props = BoxAttributes;

export function Runs({ ...rest }: Props) {
	const runs = useSyncExternalStore(runner.state.subscribe, runner.state.getSnapshot);
	return (
		<Pane name="Runs" {...rest}>
			<Box flexDirection="column">
				{runs.map((run) => (
					<Box flexDirection="row" gap={1}>
						<Indicator status={run.status} />
						<Text>
							{`(${run.task.cwd}) ${run.task.command}`} ${run.task.args?.join(" ")}
						</Text>
					</Box>
				))}
			</Box>
		</Pane>
	);
}

type IndicatorProps = { status: RunStatus };
function Indicator({ status }: IndicatorProps) {
	switch (status) {
		case "running":
			return (
				<Text color="blue">
					<Spinner type="dots" />
				</Text>
			);
		case "succeeded":
			return <Text color="green">o</Text>;
		default:
			return <Text color="red">!</Text>;
	}
}
Indicator satisfies React.FC<IndicatorProps>;

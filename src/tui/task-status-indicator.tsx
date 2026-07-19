import { RunStatus } from "#app/core/runner.js";
import { Text } from "ink";
import Spinner from "ink-spinner";

type Props = { status: RunStatus };
export function TaskStatusIndicator({ status }: Props) {
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
TaskStatusIndicator satisfies React.FC<Props>;

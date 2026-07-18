import { Task } from "#app/core/provider.js";
import { Box, Text } from "ink";
import { BoxAttributes } from "./ui/box-attributes.js";
import { Pane } from "./ui/pane.js";

type Props = {
	task: Task | null;
} & BoxAttributes;

export function TaskDetail({ task, ...rest }: Props) {
	return (
		<Pane name="Detail" {...rest}>
			{task != null && (
				<Box flexDirection="column">
					<Property name="directory" value={task.cwd} />
					<Property name="name" value={task.name} />
					<Property name="command" value={`${task.command} ${task.args?.join(" ") ?? ""}`} />
				</Box>
			)}
		</Pane>
	);
}
TaskDetail satisfies React.FC<Props>;

type PropertyProps = { name: string; value: string };
function Property({ name, value }: PropertyProps) {
	return (
		<Box flexDirection="row">
			<Text>{name}</Text>
			<Text>: </Text>
			<Text>{value}</Text>
		</Box>
	);
}
Property satisfies React.FC<PropertyProps>;

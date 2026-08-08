import { iife } from "#app/utils/iife.js";
import { Box, Text } from "ink";
import { BoxAttributes } from "./box-attributes.js";

type Props = {
	name: React.ReactNode;
	children: React.ReactNode;
} & BoxAttributes;

export function Pane({ name, children, ...rest }: Props) {
	return (
		<Box
			borderStyle="round"
			borderColor="gray"
			position="relative"
			flexDirection="column"
			{...rest}
		>
			<Box
				marginTop={-1}
				marginLeft={2}
				flexGrow={0}
				flexShrink={0}
				position="absolute"
			>
				{iife(() => {
					switch (typeof name) {
						case "string":
						case "number":
						case "bigint":
						case "boolean":
							return <Text>{name}</Text>;
						default:
							return name;
					}
				})}
			</Box>
			<Box flexGrow={1} flexShrink={1} overflow="hidden">
				{children}
			</Box>
		</Box>
	);
}
Pane satisfies React.FC<Props>;

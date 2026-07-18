import { Text } from "ink";
import { BoxAttributes } from "./ui/box-attributes.js";
import { Pane } from "./ui/pane.js";

type Props = BoxAttributes;

export function Runs({ ...rest }: BoxAttributes) {
	return (
		<Pane name="Runs" {...rest}>
			<Text>runs here</Text>
		</Pane>
	);
}

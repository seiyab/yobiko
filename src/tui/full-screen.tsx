// https://github.com/vadimdemedes/ink/issues/263#issuecomment-1926026985

import { Box, render, type RenderOptions, useStdout } from "ink";
import React from "react";

function useStdoutDimensions(): [number, number] {
	const { stdout } = useStdout();
	const { columns, rows } = stdout;
	const [size, setSize] = React.useState({ columns, rows });
	React.useLayoutEffect(() => {
		function onResize() {
			const { columns, rows } = stdout;
			setSize({ columns, rows });
		}

		stdout.on("resize", onResize);
		return () => {
			stdout.off("resize", onResize);
		};
	}, [stdout]);
	return [size.columns, size.rows];
}

function FullScreen({ children }: React.PropsWithChildren): React.ReactNode {
	const [columns, rows] = useStdoutDimensions();
	return (
		<Box width={columns} height={rows}>
			{children}
		</Box>
	);
}

export const renderFullScreen = (element: React.ReactNode, options?: RenderOptions) => {
	process.stdout.write("\x1b[?1049h");
	const instance = render(<FullScreen>{element}</FullScreen>, options);
	void instance.waitUntilExit().then(() => process.stdout.write("\x1b[?1049l"));
	return instance;
};

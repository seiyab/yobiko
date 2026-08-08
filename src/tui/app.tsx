import { useState } from "react";
import { useApp, Box, Text } from "ink";
import { useQuery } from "@tanstack/react-query";
import { ecq } from "@seiyab/ecq";
import { setup } from "#app/core/scan.js";
import { npm } from "#app/providers/npm/provider.js";
import { mise } from "#app/providers/mise/provider.js";
import { uv } from "#app/providers/uv/provider.js";
import Spinner from "ink-spinner";
import { Launcher } from "./pages/launcher.js";
import { iife } from "#app/utils/iife.js";
import { History } from "./pages/history.js";
import { useFocus, useKeyMap } from "./input.js";
import { OneShot } from "./pages/one-shot.js";
import { Help } from "./help.js";

const s = ecq.client(setup([npm, mise, uv]));

export function App() {
	const ws = useQuery(s.scan("./", { depth: 3 }));
	const [mode, setMode] = useState<"one-shot" | "dashboard">("one-shot");
	const [tab, setTab] = useState<"launcher" | "project" | "history">("launcher");
	const helpFocus = useFocus();
	const { exit } = useApp();
	useKeyMap({
		q: { action: exit, description: "exit from yobiko" },
	});
	useKeyMap(
		mode === "one-shot" && {
			"<c-d>": {
				action: () => setMode("dashboard"),
				description: "switch to dashboard",
			},
		},
	);
	useKeyMap(
		mode === "dashboard" && {
			L: {
				action: () => setTab("launcher"),
				description: "open launcher view",
			},
			H: {
				action: () => setTab("history"),
				description: "open history view",
			},
		},
	);
	useKeyMap({
		"?": {
			action: () => {
				if (helpFocus.active) {
					helpFocus.release();
				} else {
					helpFocus.capture();
				}
			},
			description: "toggle help",
			active: ({ focusID }) => focusID == null || focusID === helpFocus.id,
		},
	});
	return (
		<Box flexDirection="column" alignItems="stretch" width="100%" height="100%">
			{mode === "dashboard" && (
				<Box flexDirection="row" gap={3} height={1} flexGrow={0} flexBasis={1} flexShrink={0}>
					<Text underline={tab == "launcher"}>[L]auncher</Text>
					<Text underline={tab == "project"}>[P]roject</Text>
					<Text underline={tab == "history"}>[H]istory</Text>
				</Box>
			)}
			{ws.isPending ? (
				<Box>
					<Spinner type="bouncingBar" />
					<Text>loading project...</Text>
				</Box>
			) : ws.isError ? (
				<Text>error</Text>
			) : (
				iife(() => {
					if (mode === "one-shot") return <OneShot workspaces={ws.data} />;
					switch (tab) {
						case "launcher":
							return <Launcher workspaces={ws.data} />;
						case "project":
							return <Text>not implemented yet</Text>;
						case "history":
							return <History />;
						default:
							tab satisfies never;
					}
				})
			)}
			<Box flexGrow={0} flexShrink={0}>
				<Text>Yobiko</Text>
			</Box>
			{helpFocus.active && <Help />}
		</Box>
	);
}

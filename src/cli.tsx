#!/usr/bin/env node
import React from "react";
import { App } from "./tui/app.js";
import { renderFullScreen } from "./tui/full-screen.js";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { KeyMapListener } from "./tui/input.js";
import { maybeAttachAndRun } from "./core/one-shot-runner.js";

const queryClient = new QueryClient();

const app = renderFullScreen(
	<QueryClientProvider client={queryClient}>
		<App />
		<KeyMapListener />
	</QueryClientProvider>,
);

await app.waitUntilExit();

maybeAttachAndRun();

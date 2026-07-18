#!/usr/bin/env node
import React from "react";
import { App } from "./tui/app.js";
import { renderFullScreen } from "./tui/full-screen.js";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient();

renderFullScreen(
	<QueryClientProvider client={queryClient}>
		<App />
	</QueryClientProvider>,
);

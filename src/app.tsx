import React from "react";
import { useApp, Box, Text } from "ink";

export function App() {
  const { exit } = useApp();
  return (
    <Box flexDirection="column" alignItems="stretch" width="100%" height="100%">
      <Text>Hello</Text>
    </Box>
  );
}

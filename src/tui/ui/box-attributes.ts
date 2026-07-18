import { Box } from "ink";
import { ComponentProps } from "react";

export type BoxAttributes = Omit<ComponentProps<typeof Box>, "children">;

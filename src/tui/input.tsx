import { counter } from "#app/utils/counter.js";
import { useInput } from "ink";
import { useEffect, useMemo, useState, useSyncExternalStore } from "react";
import { newState } from "./state.js";

type Focus = {
	id: FocusID;
	active: boolean;
	capture: () => void;
	release: () => void;
};

type KeyAction = {
	action: () => void;
	description: string;
	active?: (state: ActiveFunctionParameter) => boolean;
};
type LocalKeyMap = Partial<Record<string, KeyAction>>;
export type ActiveFunctionParameter = { focusID: FocusID | null };

type ComponentID = number;
type FocusID = number;

type Help = Record<string, KeyAction>;

function newKeyMap() {
	const componentIDs = counter();
	const registrations = newState<Record<ComponentID, LocalKeyMap>>({});

	const focus = newState<FocusID | null>(null);

	return {
		KeyMapListener,
		useKeyMap,
		useFocus,
		useHelp,
	};

	function KeyMapListener() {
		useInput((input, key) => {
			const keyMap = registrations.getSnapshot();
			switch (true) {
				case key.return:
					handle("<return>");
					return;
				case key.tab:
					handle("<tab>");
					return;
				case key.escape:
					handle("<esc>");
					return;
				default:
				// nop
			}
			for (const c of input) {
				const k = key.ctrl ? `<c-${c}>` : c;
				handle(k);
			}

			function handle(bind: string) {
				for (const km of Object.values(keyMap)) {
					const m = km[bind];
					if (m == null) continue;
					const p = m.active ?? defaultActiveFunction;
					if (!p({ focusID: focus.getSnapshot() })) return;
					m?.action();
				}
			}
		});
		return null;
	}

	function useKeyMap(keyMap: LocalKeyMap | false) {
		const [id] = useState(() => componentIDs.next());

		useEffect(() => {
			if (!keyMap) return;
			registrations.update((prev) => ({
				...prev,
				[id]: keyMap,
			}));

			return () => {
				registrations.update((prev) =>
					Object.fromEntries(Object.entries(prev).filter(([key]) => key != String(id))),
				);
			};
		}, [id, keyMap]);
	}

	function useFocus(): Focus {
		const [id] = useState(() => componentIDs.next());
		const currentFocus = useSyncExternalStore(focus.subscribe, focus.getSnapshot);
		useEffect(
			() => () => {
				const f = focus.getSnapshot();
				if (f === id) {
					focus.update(() => null);
				}
			},
			[id],
		);

		return useMemo(
			() => ({
				id,
				active: currentFocus === id,
				capture: () => focus.update(() => id),
				release: () =>
					focus.update((prev) => {
						if (prev === id) return null;
						return prev;
					}),
			}),
			[currentFocus, id],
		);
	}

	function useHelp(): Help {
		const rs = useSyncExternalStore(registrations.subscribe, registrations.getSnapshot);
		return useMemo(
			() =>
				Object.fromEntries(
					Object.values(rs).flatMap((r) =>
						Object.entries(r).filter((e): e is [string, KeyAction] => e[1] !== undefined),
					),
				),
			[rs],
		);
	}
}

export const { KeyMapListener, useKeyMap, useFocus, useHelp } = newKeyMap();

function defaultActiveFunction({ focusID }: ActiveFunctionParameter): boolean {
	return focusID == null;
}

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
	focus?: FocusID;
};
type LocalKeyMap = Partial<Record<string, KeyAction>>;

type ComponentID = number;
type FocusID = number;

type Help = Record<string, KeyAction>;

function newKeyMap() {
	const componentIDs = counter();
	const registrations = newState<Record<ComponentID, LocalKeyMap>>({});

	const focus = newState<FocusID | null>(null);

	return {
		useKeyMap,
		useFocus,
		useHelp,
	};

	function useKeyMap(keyMap: LocalKeyMap) {
		const [id] = useState(() => componentIDs.next());

		useInput((input, key) => {
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
				const m = keyMap[bind];
				if (m == null) return;
				// const focusID = focus.getSnapshot();
				// if (focusID != null && m.focus !== focusID) return;
				m?.action();
			}
		});

		useEffect(() => {
			registrations.update((prev) => ({
				...prev,
				[id]: keyMap,
			}));

			return () => {
				registrations.update((prev) =>
					Object.fromEntries(
						Object.entries(prev).filter(([key]) => key != String(id)),
					),
				);
			};
		}, [id, keyMap]);
	}

	function useFocus(): Focus {
		const [id] = useState(() => componentIDs.next());
		const currentFocus = useSyncExternalStore(
			focus.subscribe,
			focus.getSnapshot,
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
		const rs = useSyncExternalStore(
			registrations.subscribe,
			registrations.getSnapshot,
		);
		return useMemo(
			() =>
				Object.fromEntries(
					Object.values(rs).flatMap((r) =>
						Object.entries(r).filter(
							(e): e is [string, KeyAction] => e[1] !== undefined,
						),
					),
				),
			[rs],
		);
	}
}

export const { useKeyMap, useFocus, useHelp } = newKeyMap();

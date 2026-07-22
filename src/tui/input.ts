import { counter } from "#app/utils/counter.js";
import { useInput } from "ink";
import { useEffect, useMemo, useState, useSyncExternalStore } from "react";

type Focus = {
	active: boolean;
	capture: () => void;
	release: () => void;
};

type KeyAction = {
	action: () => void;
	description: string;
};
type LocalKeyMap = Partial<Record<string, KeyAction>>;

type ComponentID = number;

type Help = Record<string, KeyAction>;

function newKeyMap() {
	const componentIDs = counter();
	let registrations: Record<ComponentID, LocalKeyMap> = {};
	const registrationSubs = new Set<() => void>();

	let focus: ComponentID | null = null;
	const focusSubs = new Set<() => void>();

	return {
		useKeyMap,
		useFocus,
		useHelp,
	};

	function useKeyMap(keyMap: LocalKeyMap) {
		const [id] = useState(() => componentIDs.next());

		useInput((input, key) => {
			if (focus != null) return;
			switch (true) {
				case key.return:
					keyMap["<return>"]?.action();
					break;
				default:
				// nop
			}
			for (const c of input) {
				const k = key.ctrl ? `<c-${c}>` : c;
				keyMap[k]?.action();
			}
		});

		useEffect(() => {
			registrations = {
				...registrations,
				[id]: keyMap,
			};
			emitRegistration();

			return () => {
				registrations = Object.fromEntries(
					Object.entries(registrations).filter(([key]) => key != String(id)),
				);
				emitRegistration();
			};
		}, [id, keyMap]);

		function emitRegistration() {
			for (const s of registrationSubs) {
				s();
			}
		}
	}

	function useFocus(): Focus {
		const [id] = useState(() => componentIDs.next());
		const currentFocus = useSyncExternalStore(subscribe, getSnapshot);

		return useMemo(
			() => ({
				active: currentFocus === id,
				capture: () => {
					if (focus === id) return;
					focus = id;
					emitFocus();
				},
				release: () => {
					if (focus !== id) return;
					focus = null;
					emitFocus();
				},
			}),
			[currentFocus],
		);

		function getSnapshot() {
			return focus;
		}
		function subscribe(listener: () => void): () => void {
			focusSubs.add(listener);
			return () => focusSubs.delete(listener);
		}

		function emitFocus() {
			for (const s of focusSubs) {
				s();
			}
		}
	}

	function useHelp(): Help {
		const rs = useSyncExternalStore(subscribe, getSnapshot);
		return useMemo(
			() =>
				Object.fromEntries(
					Object.values(rs).flatMap((r) =>
						Object.entries(r).filter((e): e is [string, KeyAction] => e[1] !== undefined),
					),
				),
			[registrations],
		);

		function getSnapshot() {
			return registrations;
		}
		function subscribe(listener: () => void): () => void {
			registrationSubs.add(listener);
			return () => registrationSubs.delete(listener);
		}
	}
}

export const { useKeyMap, useFocus, useHelp } = newKeyMap();

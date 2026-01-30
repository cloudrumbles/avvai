import { getContext, setContext } from 'svelte';
import type { ComponentType } from 'svelte';

export type SidebarOverride = {
	structureComponent?: ComponentType;
	structureProps?: Record<string, unknown>;
	structureTitle?: string;
	structureCount?: number;
	fullWidth?: boolean;
};

type LayoutContext = {
	setSidebarOverride: (override: SidebarOverride) => void;
	clearSidebarOverride: () => void;
};

const LAYOUT_CONTEXT_KEY = Symbol('layout-context');

export function setLayoutContext(ctx: LayoutContext) {
	setContext(LAYOUT_CONTEXT_KEY, ctx);
}

export function useLayoutContext(): LayoutContext {
	const ctx = getContext<LayoutContext>(LAYOUT_CONTEXT_KEY);
	if (!ctx) {
		throw new Error('Layout context is not available');
	}
	return ctx;
}

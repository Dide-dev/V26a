import { writable } from 'svelte/store';
import type { Node, RootNode } from '../tree-viewer/example_tree';

export const selectedNode = writable<Node | RootNode | null>(null);
export const secondSelectedNode = writable<Node | RootNode | null>(null);


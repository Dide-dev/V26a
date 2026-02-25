import { writable } from 'svelte/store';
import type { Node } from '../tree-viewer/example_tree';

export const selectedNode = writable<Node | null>(null);
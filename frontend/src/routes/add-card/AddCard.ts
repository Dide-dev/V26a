import type { Node } from '../tree-viewer/example_tree';

//adds a new child node
export function addChild(parent: Node, text: string, onAdd: () => void) {
    parent.children = [...parent.children, { card: text, children: [] }];
    onAdd();
}

//insterts a new child between two nodes
export function insertChild(parent: Node, child: Node, text: string, onAdd: () => void) {
    const newNode: Node = { card: text, children: [child] };
    parent.children = parent.children.map(c => c === child ? newNode : c) as Node[];
    onAdd();
}

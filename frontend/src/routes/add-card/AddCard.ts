import type { Node } from '../tree-viewer/example_tree';

//adds a new child node
export function addChild(parent: Node, front: string, back: string, onAdd: (front: string, back: string) => void) {
    parent.children = [...parent.children, { card: back, children: [] }];
    onAdd(front, back);
}

//insterts a new child between two nodes
export function insertChild(parent: Node, child: Node, front: string, back: string, onAdd: (front: string, back: string) => void) {
    const newNode: Node = { card: back, children: [child] };
    parent.children = parent.children.map(c => c === child ? newNode : c) as Node[];
    onAdd(front, back);
}

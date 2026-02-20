// For now just an example, TODO: getting tree from database
type Tree = {
	name: string;
	// assignment, meta data, ...
	roots: { root_text: string; root_nodes: Node[] }[];
};
type Node = {
	card: Card;
	children: Node[];
};
type Card = string;

export const example_tree: Tree = {
	name: 'Testing 123',
	roots: [
		{
			root_text: 'Oriënteren',
			root_nodes: [
				{
					card: 'Lees de opdracht helemaal door!',
					children: []
				},
				{
					card: 'De belangrijke gegevens zijn a en b',
					children: []
				}
			]
		},
		{
			root_text: 'Plan maken en uitvoeren',
			root_nodes: [
				{
					card: 'Zoek het verband tussen x en y, bijvoorbeeld delers',
					children: [
						{
							card: 'Wat als x priem is?',

							children: [
								{
									card: 'Dan is het makkelijk op te lossen!',
									children: []
								}
							]
						},
						{
							card: 'Wat als y priem is?',

							children: [
								{
									card: 'Deel 1 kan dan op deze manier',
									children: []
								},
								{
									card: 'Deel 2 kan dan eindelijk ook',
									children: []
								}
							]
						}
					]
				}
			]
		}
	]
};

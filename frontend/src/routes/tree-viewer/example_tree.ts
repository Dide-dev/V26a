// For now just an example, TODO: getting tree from database
export type HeuristicTree = {
	name: string;
	// assignment, meta data, ...
	roots: RootNode[];
};
export type RootNode = {
	label: string;
	children: Node[];
};
export type Node = {
	card: Card;
	children: Node[];
};
export type Card = string;

export const example_tree: HeuristicTree = {
	name: 'Testing 123',
	roots: [
		{
			label: 'Oriënteren',
			children: [
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
			label: 'Plan maken en uitvoeren',
			children: [
				{
					card: 'Zoek het verband tussen x en y, bijvoorbeeld delers',
					children: [
						{
							card: 'Wat als x priem is?',

							children: [
								{
									card: 'Dan is het makkelijk op te lossen!',
									children: []
								},
								{
									card: 'abc',
									children: [
										{
											card: '123',
											children: []
										},
										{
											card: '456',
											children: []
										}
									]
								},
								{
									card: 'def',
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

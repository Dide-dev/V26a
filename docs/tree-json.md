[This program has been developed by students from the bachelor Computer Science at
Utrecht University within the Software Project course.
© Copyright Utrecht University (Department of Information and Computing Sciences)]: #

In the database table `trees` there is a field `structure` which is of type `JSONB`.
The format of the JSON will look like this:

```json
{
    "rootNodes": [
        {
            "label": "Orientation",
            "children": [
                {
                    "cardId": 42,
                    "children": [
                        {
                            "cardId": 8,
                            "children": []
                        },
                        {
                            "cardId": 44,
                            "children": [
                                {
                                    "cardId": 101,
                                    "children": []
                                },
                                {
                                    "cardId": 18,
                                    "children": []
                                }
                            ]
                        },
                        {
                            "cardId": 23,
                            "children": []
                        }
                    ]
                },
                ...
            ]
        },
        {
            "label": "Making & executing plans",
            "children": [
                ...
            ]
        },
        ...
    ]
}
```

Make sure to differentiate the 'root nodes' from the normal 'nodes' in the types in rust and TypeScript!

The `cardId`'s correspond to the id in the cards database table.

Cards may exist without their card id present in the JSON structure.
This is because when adding a new card the frontend initially does not know the id,
so the frontend can not create the new structure yet.

A card id that does not (yet) exist in the cards database table is **NOT** allowed,
and considered to be an invalid database state (just like when the JSON structure has wrong keys).

A card id may **NOT** occur more than once in any tree, and any such occurrence would be considered an invalid database
state.

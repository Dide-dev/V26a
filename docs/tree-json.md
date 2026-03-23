*This program has been developed by students from the bachelor Computer Science at*
*Utrecht University within the Software Project course.*
*© Copyright Utrecht University (Department of Information and Computing Sciences)*

In the database table `trees` there is a field `structure` which is of type `JSONB`.
The format of the json will look like this:
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
                                },
                            ],
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

Make sure to differentiate the 'root nodes' from the normal 'nodes' in the types in rust and typescript!

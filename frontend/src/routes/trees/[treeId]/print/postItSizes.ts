export class PostItSize {
    /** Width in millimeters. */
    width: number = 76;
    /** Height in millimeters. */
    height: number = 76;

    toString() {
        return `${this.width}mm x ${this.height}mm`;
    }

    constructor(init: Partial<PostItSize>) {
        Object.assign(this, init);
    }
}

export const defaultPostItSizes: PostItSize[] = [
    new PostItSize({ width: 47.6, height: 47.6 }),
    new PostItSize({ width: 76, height: 76 }),
    new PostItSize({ width: 101, height: 101 })
];

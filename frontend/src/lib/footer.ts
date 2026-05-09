/** Data used to render a link in the footer */
export class FooterLink {
    /** Href linked by the link. */
    href: string;
    /** Text to display for the link. */
    text: string;
    /**
     * Target page when clicking link.
     * @default '_self'
     */
    target: string = '_self';
    /**
     * Boolean indicating whether the href should be localized.
     *
     * @default true
     * @remarks Should only be `false` if the link is outside the main website architecture.
     */
    localized: boolean = true;

    /**
     * @param {FooterLink} init - Required values are {@link href} and {@link text},
     *                            and optional values are {@link target} and {@link localized}.
     */
    constructor(init: Pick<FooterLink, 'href' | 'text'> & Partial<FooterLink>) {
        this.href = init.href;
        this.text = init.text;
        Object.assign(this, init);
    }
}

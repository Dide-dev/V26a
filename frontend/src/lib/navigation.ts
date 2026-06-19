/** Data used to render a link in the navigation bar. */
export class NavLink {
    /**
     * Href linked by the top-level nav-bar item.
     *
     * @remarks If this link has children, this href **must** also be included in {@link children},
     * otherwise it will **not** be rendered on smaller screens.
     */
    defaultHref: string;
    /** Text to display on the top-level nav-bar item. */
    text: string;
    /**
     * Optional properties for nav-bar item.
     * @default { width: 'md', localeSwitch: false }
     */
    props: NavLinkProps = { width: 'md', localeSwitch: false };
    /** Optional children to display when drop-down is open. */
    children: NavChildLink[] = [];

    /**
     * @param {NavLink} init - Required values are {@link defaultHref} and {@link text},
     *                         and optional values are {@link props} and {@link children}.
     *
     * @remarks If this link has children, the {@link defaultHref} **must** also be included in
     * {@link children}, otherwise it will **not** be rendered on smaller screens.
     * */
    constructor(init: Pick<NavLink, 'defaultHref' | 'text'> & Partial<NavLink>) {
        this.defaultHref = init.defaultHref;
        this.text = init.text;
        Object.assign(this, init);
    }
}

/**
 * Data used to render a drop-down link in the nav-bar.
 * @param {string} href - The relative URL to link to.
 * @param {string} text - The text to display on the link.
 */
export type NavChildLink = {
    href: string;
    text: string;
};

/**
 * Extra data to alter the way a nav-bar item is rendered.
 * @param {NavWidthProp} width - The width of the nav-bar item.
 * @param {boolean} localeSwitch - Flag indicating whether this nav-bar item invokes a locale switch.
 *                                 If `true`, it will trigger a reload of the page.
 */
export type NavLinkProps = {
    width: NavWidthProp;
    localeSwitch: boolean;
};

/**
 * Property defining the width of a nav-bar item. Possible values:
 * - `'lg'` - Large width.
 * - `'md'` - Medium width.
 * - `'th'` - Thin width.
 * - `'xt'` - Extra thin width.
 */
export type NavWidthProp = 'lg' | 'md' | 'th' | 'xt';

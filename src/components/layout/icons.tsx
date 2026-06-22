import type { SVGProps } from "react";

// Inline icon set for the sidebar. No external icon library is used in the
// first version to avoid introducing unconfirmed dependencies. The icons are
// intentionally simple geometric glyphs that fit the Material Design 3 dark
// direction described in docs/08_page_wireframes.md.

type IconProps = SVGProps<SVGSVGElement>;

function baseIconProps(props: IconProps): IconProps {
  return {
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    strokeWidth: 1.6,
    strokeLinecap: "round",
    strokeLinejoin: "round",
    "aria-hidden": true,
    focusable: false,
    ...props,
  };
}

// Home: a small spark / star to represent the start page.
export function HomeIcon(props: IconProps) {
  return (
    <svg {...baseIconProps(props)}>
      <path d="M12 3.2l2.1 4.9 5.3.4-4.1 3.4 1.3 5.2L12 14.8 7.4 17.1l1.3-5.2-4.1-3.4 5.3-.4z" />
    </svg>
  );
}

// Actresses: a portrait / avatar silhouette.
export function ActressIcon(props: IconProps) {
  return (
    <svg {...baseIconProps(props)}>
      <circle cx="12" cy="8.4" r="3.4" />
      <path d="M5.6 19.6c.7-3.4 3.4-5.4 6.4-5.4s5.7 2 6.4 5.4" />
    </svg>
  );
}

// Videos: a rectangular media / play card.
export function VideoIcon(props: IconProps) {
  return (
    <svg {...baseIconProps(props)}>
      <rect x="4" y="5.4" width="16" height="13.2" rx="2.4" />
      <path d="M10.4 9.7l4.1 2.3-4.1 2.3z" />
    </svg>
  );
}

// Settings: a gear.
export function SettingsIcon(props: IconProps) {
  return (
    <svg {...baseIconProps(props)}>
      <circle cx="12" cy="12" r="3.1" />
      <path d="M12 3.4l1 2.2 2.4-.5.3 2.4 2.2 1-.9 2.2.9 2.2-2.2 1-.3 2.4-2.4-.5-1 2.2-1-2.2-2.4.5-.3-2.4-2.2-1 .9-2.2-.9-2.2 2.2-1 .3-2.4 2.4.5z" />
    </svg>
  );
}

// Account: a person avatar used at the sidebar bottom.
export function AccountIcon(props: IconProps) {
  return (
    <svg {...baseIconProps(props)}>
      <circle cx="12" cy="9" r="3.2" />
      <path d="M6 19.4c.8-3.2 3.2-5 6-5s5.2 1.8 6 5" />
    </svg>
  );
}

// Collapse: a chevron pointing left, shown in the expanded sidebar header.
export function CollapseIcon(props: IconProps) {
  return (
    <svg {...baseIconProps(props)}>
      <path d="M14.5 6L9 12l5.5 6" />
    </svg>
  );
}

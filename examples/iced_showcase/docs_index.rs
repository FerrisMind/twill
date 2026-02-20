use iced::widget::operation::RelativeOffset;

use crate::pages::Page;

#[derive(Debug, Clone, Copy)]
pub struct TocEntry {
    pub title: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct ShowcaseEntry {
    pub page: Page,
    pub title: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub toc: &'static [TocEntry],
}

#[derive(Debug, Clone, Copy)]
pub struct DocsNavEntry {
    pub title: &'static str,
    pub path: &'static str,
    pub page: Option<Page>,
    pub children: &'static [DocsNavEntry],
}

#[derive(Debug, Clone, Copy)]
pub struct DocsCategory {
    pub title: &'static str,
    pub entries: &'static [DocsNavEntry],
}

const TYPOGRAPHY_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Type scale",
    },
    TocEntry { title: "Usage" },
];

const COLORS_TAILWIND_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Palette families",
    },
    TocEntry { title: "Usage" },
];

const COLORS_SEMANTIC_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Semantic tokens",
    },
    TocEntry { title: "Usage" },
];

const OKLCH_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "OKLCH controls",
    },
    TocEntry { title: "Examples" },
];

const SPACING_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry { title: "Scale" },
    TocEntry { title: "Examples" },
];

const ASPECT_RATIO_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry { title: "Square" },
    TocEntry { title: "Video" },
    TocEntry { title: "Custom" },
];

const COLUMNS_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Setting by number",
    },
    TocEntry {
        title: "Setting by width",
    },
    TocEntry {
        title: "Setting maximum column count",
    },
    TocEntry {
        title: "Setting the column gap",
    },
    TocEntry {
        title: "Auto behavior",
    },
    TocEntry {
        title: "Using a custom value",
    },
];

const OVERFLOW_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Auto overflow",
    },
];

const DISPLAY_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Hidden display",
    },
];

const FLEX_DIRECTION_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Class coverage",
    },
    TocEntry {
        title: "Row (`flex-row`)",
    },
    TocEntry {
        title: "Row reverse (`flex-row-reverse`)",
    },
    TocEntry {
        title: "Column (`flex-col`)",
    },
    TocEntry {
        title: "Column reverse (`flex-col-reverse`)",
    },
];

const FLEX_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Class coverage",
    },
    TocEntry {
        title: "Number (`flex-<number>`)",
    },
    TocEntry {
        title: "Fraction (`flex-<fraction>`)",
    },
    TocEntry {
        title: "Initial (`flex-initial`)",
    },
    TocEntry {
        title: "Auto (`flex-auto`)",
    },
    TocEntry {
        title: "None (`flex-none`)",
    },
    TocEntry {
        title: "Custom property (`flex-(<custom-property>)`)",
    },
    TocEntry {
        title: "Arbitrary (`flex-[<value>]`)",
    },
];

const OBJECT_FIT_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Iced mapping",
    },
];

const MAX_WIDTH_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Prose width",
    },
];

const BORDERS_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Radius scale",
    },
    TocEntry {
        title: "Border styles",
    },
];

const SHADOWS_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Shadow scale",
    },
    TocEntry { title: "Examples" },
];

const MOTION_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Motion primitives",
    },
    TocEntry { title: "Live demo" },
];

const BUTTONS_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry { title: "Variants" },
    TocEntry { title: "States" },
];

const STYLE_BUILDER_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry { title: "Controls" },
    TocEntry { title: "Output" },
];

const KITCHEN_SINK_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry {
        title: "Composite layout",
    },
    TocEntry { title: "Full demo" },
];

const TRANSLATOR_TOC: &[TocEntry] = &[
    TocEntry { title: "Overview" },
    TocEntry { title: "Inputs" },
    TocEntry { title: "Output" },
];

const SHOWCASE_ENTRIES: &[ShowcaseEntry] = &[
    ShowcaseEntry {
        page: Page::Typography,
        title: "Typography",
        description: "Type scale, rhythm, and practical text styling with Twill tokens.",
        category: "Typography",
        toc: TYPOGRAPHY_TOC,
    },
    ShowcaseEntry {
        page: Page::Colors,
        title: "Colors (Tailwind)",
        description: "Complete Tailwind-inspired palettes from neutral ramps to vivid accents.",
        category: "General",
        toc: COLORS_TAILWIND_TOC,
    },
    ShowcaseEntry {
        page: Page::SemanticColors,
        title: "Colors (Semantic)",
        description: "Intent-driven semantic colors for surfaces, text, and stateful UI.",
        category: "General",
        toc: COLORS_SEMANTIC_TOC,
    },
    ShowcaseEntry {
        page: Page::OklchDemo,
        title: "OKLCH Demonstration",
        description: "Perceptual color operations for cleaner hover, active, and generated shades.",
        category: "General",
        toc: OKLCH_TOC,
    },
    ShowcaseEntry {
        page: Page::Spacing,
        title: "Spacing",
        description: "Consistent spacing scale for composition, rhythm, and alignment.",
        category: "Spacing",
        toc: SPACING_TOC,
    },
    ShowcaseEntry {
        page: Page::Borders,
        title: "Borders",
        description: "Border radii, widths, and outlines for crisp component boundaries.",
        category: "Borders",
        toc: BORDERS_TOC,
    },
    ShowcaseEntry {
        page: Page::Shadows,
        title: "Shadows",
        description: "Depth tokens and layering guidance for surfaces and elevation.",
        category: "Effects",
        toc: SHADOWS_TOC,
    },
    ShowcaseEntry {
        page: Page::AspectRatio,
        title: "aspect-ratio",
        description: "Utilities for controlling the aspect ratio of an element.",
        category: "Layout",
        toc: ASPECT_RATIO_TOC,
    },
    ShowcaseEntry {
        page: Page::Columns,
        title: "columns",
        description: "Utilities for controlling the number of columns within an element.",
        category: "Layout",
        toc: COLUMNS_TOC,
    },
    ShowcaseEntry {
        page: Page::Overflow,
        title: "overflow",
        description: "Utilities for controlling how an element handles overflow content.",
        category: "Layout",
        toc: OVERFLOW_TOC,
    },
    ShowcaseEntry {
        page: Page::Display,
        title: "display",
        description: "Utilities for controlling the display box type of an element.",
        category: "Layout",
        toc: DISPLAY_TOC,
    },
    ShowcaseEntry {
        page: Page::FlexDirection,
        title: "flex-direction",
        description: "Utilities for controlling the direction of flex items.",
        category: "Flexbox & Grid",
        toc: FLEX_DIRECTION_TOC,
    },
    ShowcaseEntry {
        page: Page::Flex,
        title: "flex",
        description: "Utilities for controlling how flex items both grow and shrink.",
        category: "Flexbox & Grid",
        toc: FLEX_TOC,
    },
    ShowcaseEntry {
        page: Page::ObjectFit,
        title: "object-fit",
        description: "Utilities for controlling how replaced content should be resized to fit its container.",
        category: "Layout",
        toc: OBJECT_FIT_TOC,
    },
    ShowcaseEntry {
        page: Page::MaxWidth,
        title: "max-width",
        description: "Utilities for setting the maximum width of an element.",
        category: "Sizing",
        toc: MAX_WIDTH_TOC,
    },
    ShowcaseEntry {
        page: Page::Buttons,
        title: "Buttons",
        description: "Button variants and states built with reusable Twill styles.",
        category: "Showcase",
        toc: BUTTONS_TOC,
    },
    ShowcaseEntry {
        page: Page::Motion,
        title: "Motion & Animation",
        description: "Motion tokens and animation previews for interactive state changes.",
        category: "Transitions & Animation",
        toc: MOTION_TOC,
    },
    ShowcaseEntry {
        page: Page::StyleBuilder,
        title: "Style Builder",
        description: "Composable styling API examples for building consistent UI patterns.",
        category: "Showcase",
        toc: STYLE_BUILDER_TOC,
    },
    ShowcaseEntry {
        page: Page::KitchenSink,
        title: "Kitchen Sink",
        description: "Full-page composition that combines multiple tokens in one layout.",
        category: "Showcase",
        toc: KITCHEN_SINK_TOC,
    },
    ShowcaseEntry {
        page: Page::Translator,
        title: "Real World: Translator",
        description: "A practical screen that demonstrates Twill styling in a real workflow.",
        category: "Showcase",
        toc: TRANSLATOR_TOC,
    },
];

const DOCS_NAV_CATEGORIES: &[DocsCategory] = &[
    DocsCategory {
        title: "General",
        entries: &[
            DocsNavEntry {
                title: "Styling with utility classes",
                path: "/docs/styling-with-utility-classes",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "Hover, focus, and other states",
                path: "/docs/hover-focus-and-other-states",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "Responsive design",
                path: "/docs/responsive-design",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "Dark mode",
                path: "/docs/dark-mode",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "Theme variables",
                path: "/docs/theme",
                page: Some(Page::SemanticColors),
                children: &[],
            },
            DocsNavEntry {
                title: "Colors",
                path: "/docs/colors",
                page: Some(Page::Colors),
                children: &[],
            },
            DocsNavEntry {
                title: "Adding custom styles",
                path: "/docs/adding-custom-styles",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "Detecting classes in source files",
                path: "/docs/detecting-classes-in-source-files",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "Functions and directives",
                path: "/docs/functions-and-directives",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Layout",
        entries: &[
            DocsNavEntry {
                title: "aspect-ratio",
                path: "/docs/aspect-ratio",
                page: Some(Page::AspectRatio),
                children: &[],
            },
            DocsNavEntry {
                title: "columns",
                path: "/docs/columns",
                page: Some(Page::Columns),
                children: &[],
            },
            DocsNavEntry {
                title: "break-after",
                path: "/docs/break-after",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "break-before",
                path: "/docs/break-before",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "break-inside",
                path: "/docs/break-inside",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "box-decoration-break",
                path: "/docs/box-decoration-break",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "box-sizing",
                path: "/docs/box-sizing",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "display",
                path: "/docs/display",
                page: Some(Page::Display),
                children: &[],
            },
            DocsNavEntry {
                title: "float",
                path: "/docs/float",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "clear",
                path: "/docs/clear",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "isolation",
                path: "/docs/isolation",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "object-fit",
                path: "/docs/object-fit",
                page: Some(Page::ObjectFit),
                children: &[],
            },
            DocsNavEntry {
                title: "object-position",
                path: "/docs/object-position",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "overflow",
                path: "/docs/overflow",
                page: Some(Page::Overflow),
                children: &[],
            },
            DocsNavEntry {
                title: "overscroll-behavior",
                path: "/docs/overscroll-behavior",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "position",
                path: "/docs/position",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "top / right / bottom / left",
                path: "/docs/top-right-bottom-left",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "visibility",
                path: "/docs/visibility",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "z-index",
                path: "/docs/z-index",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Flexbox & Grid",
        entries: &[
            DocsNavEntry {
                title: "flex-basis",
                path: "/docs/flex-basis",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "flex-direction",
                path: "/docs/flex-direction",
                page: Some(Page::FlexDirection),
                children: &[],
            },
            DocsNavEntry {
                title: "flex-wrap",
                path: "/docs/flex-wrap",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "flex",
                path: "/docs/flex",
                page: Some(Page::Flex),
                children: &[],
            },
            DocsNavEntry {
                title: "flex-grow",
                path: "/docs/flex-grow",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "flex-shrink",
                path: "/docs/flex-shrink",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "order",
                path: "/docs/order",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "grid-template-columns",
                path: "/docs/grid-template-columns",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "grid-column",
                path: "/docs/grid-column",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "grid-template-rows",
                path: "/docs/grid-template-rows",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "grid-row",
                path: "/docs/grid-row",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "grid-auto-flow",
                path: "/docs/grid-auto-flow",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "grid-auto-columns",
                path: "/docs/grid-auto-columns",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "grid-auto-rows",
                path: "/docs/grid-auto-rows",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "gap",
                path: "/docs/gap",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "justify-content",
                path: "/docs/justify-content",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "justify-items",
                path: "/docs/justify-items",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "justify-self",
                path: "/docs/justify-self",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "align-content",
                path: "/docs/align-content",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "align-items",
                path: "/docs/align-items",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "align-self",
                path: "/docs/align-self",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "place-content",
                path: "/docs/place-content",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "place-items",
                path: "/docs/place-items",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "place-self",
                path: "/docs/place-self",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Spacing",
        entries: &[
            DocsNavEntry {
                title: "padding",
                path: "/docs/padding",
                page: Some(Page::Spacing),
                children: &[],
            },
            DocsNavEntry {
                title: "margin",
                path: "/docs/margin",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Sizing",
        entries: &[
            DocsNavEntry {
                title: "width",
                path: "/docs/width",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "min-width",
                path: "/docs/min-width",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "max-width",
                path: "/docs/max-width",
                page: Some(Page::MaxWidth),
                children: &[],
            },
            DocsNavEntry {
                title: "height",
                path: "/docs/height",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "min-height",
                path: "/docs/min-height",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "max-height",
                path: "/docs/max-height",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "inline-size",
                path: "/docs/inline-size",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "min-inline-size",
                path: "/docs/min-inline-size",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "max-inline-size",
                path: "/docs/max-inline-size",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "block-size",
                path: "/docs/block-size",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "min-block-size",
                path: "/docs/min-block-size",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "max-block-size",
                path: "/docs/max-block-size",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Typography",
        entries: &[
            DocsNavEntry {
                title: "font-family",
                path: "/docs/font-family",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "font-size",
                path: "/docs/font-size",
                page: Some(Page::Typography),
                children: &[],
            },
            DocsNavEntry {
                title: "font-smoothing",
                path: "/docs/font-smoothing",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "font-style",
                path: "/docs/font-style",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "font-weight",
                path: "/docs/font-weight",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "font-stretch",
                path: "/docs/font-stretch",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "font-variant-numeric",
                path: "/docs/font-variant-numeric",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "font-feature-settings",
                path: "/docs/font-feature-settings",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "letter-spacing",
                path: "/docs/letter-spacing",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "line-clamp",
                path: "/docs/line-clamp",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "line-height",
                path: "/docs/line-height",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "list-style-image",
                path: "/docs/list-style-image",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "list-style-position",
                path: "/docs/list-style-position",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "list-style-type",
                path: "/docs/list-style-type",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-align",
                path: "/docs/text-align",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "color",
                path: "/docs/color",
                page: Some(Page::OklchDemo),
                children: &[],
            },
            DocsNavEntry {
                title: "text-decoration-line",
                path: "/docs/text-decoration-line",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-decoration-color",
                path: "/docs/text-decoration-color",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-decoration-style",
                path: "/docs/text-decoration-style",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-decoration-thickness",
                path: "/docs/text-decoration-thickness",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-underline-offset",
                path: "/docs/text-underline-offset",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-transform",
                path: "/docs/text-transform",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-overflow",
                path: "/docs/text-overflow",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-wrap",
                path: "/docs/text-wrap",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "text-indent",
                path: "/docs/text-indent",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "vertical-align",
                path: "/docs/vertical-align",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "white-space",
                path: "/docs/white-space",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "word-break",
                path: "/docs/word-break",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "overflow-wrap",
                path: "/docs/overflow-wrap",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "hyphens",
                path: "/docs/hyphens",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "content",
                path: "/docs/content",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Backgrounds",
        entries: &[
            DocsNavEntry {
                title: "background-attachment",
                path: "/docs/background-attachment",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-clip",
                path: "/docs/background-clip",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-color",
                path: "/docs/background-color",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-image",
                path: "/docs/background-image",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-origin",
                path: "/docs/background-origin",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-position",
                path: "/docs/background-position",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-repeat",
                path: "/docs/background-repeat",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-size",
                path: "/docs/background-size",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Borders",
        entries: &[
            DocsNavEntry {
                title: "border-radius",
                path: "/docs/border-radius",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "border-width",
                path: "/docs/border-width",
                page: Some(Page::Borders),
                children: &[],
            },
            DocsNavEntry {
                title: "border-color",
                path: "/docs/border-color",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "border-style",
                path: "/docs/border-style",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "outline-width",
                path: "/docs/outline-width",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "outline-color",
                path: "/docs/outline-color",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "outline-style",
                path: "/docs/outline-style",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "outline-offset",
                path: "/docs/outline-offset",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Effects",
        entries: &[
            DocsNavEntry {
                title: "box-shadow",
                path: "/docs/box-shadow",
                page: Some(Page::Shadows),
                children: &[],
            },
            DocsNavEntry {
                title: "text-shadow",
                path: "/docs/text-shadow",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "opacity",
                path: "/docs/opacity",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mix-blend-mode",
                path: "/docs/mix-blend-mode",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "background-blend-mode",
                path: "/docs/background-blend-mode",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-clip",
                path: "/docs/mask-clip",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-composite",
                path: "/docs/mask-composite",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-image",
                path: "/docs/mask-image",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-mode",
                path: "/docs/mask-mode",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-origin",
                path: "/docs/mask-origin",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-position",
                path: "/docs/mask-position",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-repeat",
                path: "/docs/mask-repeat",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-size",
                path: "/docs/mask-size",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "mask-type",
                path: "/docs/mask-type",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Filters",
        entries: &[
            DocsNavEntry {
                title: "filter",
                path: "/docs/filter",
                page: None,
                children: &[
                    DocsNavEntry {
                        title: "blur",
                        path: "/docs/filter-blur",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "brightness",
                        path: "/docs/filter-brightness",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "contrast",
                        path: "/docs/filter-contrast",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "drop-shadow",
                        path: "/docs/filter-drop-shadow",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "grayscale",
                        path: "/docs/filter-grayscale",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "hue-rotate",
                        path: "/docs/filter-hue-rotate",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "invert",
                        path: "/docs/filter-invert",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "saturate",
                        path: "/docs/filter-saturate",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "sepia",
                        path: "/docs/filter-sepia",
                        page: None,
                        children: &[],
                    },
                ],
            },
            DocsNavEntry {
                title: "backdrop-filter",
                path: "/docs/backdrop-filter",
                page: None,
                children: &[
                    DocsNavEntry {
                        title: "blur",
                        path: "/docs/backdrop-filter-blur",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "brightness",
                        path: "/docs/backdrop-filter-brightness",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "contrast",
                        path: "/docs/backdrop-filter-contrast",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "grayscale",
                        path: "/docs/backdrop-filter-grayscale",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "hue-rotate",
                        path: "/docs/backdrop-filter-hue-rotate",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "invert",
                        path: "/docs/backdrop-filter-invert",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "opacity",
                        path: "/docs/backdrop-filter-opacity",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "saturate",
                        path: "/docs/backdrop-filter-saturate",
                        page: None,
                        children: &[],
                    },
                    DocsNavEntry {
                        title: "sepia",
                        path: "/docs/backdrop-filter-sepia",
                        page: None,
                        children: &[],
                    },
                ],
            },
        ],
    },
    DocsCategory {
        title: "Tables",
        entries: &[
            DocsNavEntry {
                title: "border-collapse",
                path: "/docs/border-collapse",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "border-spacing",
                path: "/docs/border-spacing",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "table-layout",
                path: "/docs/table-layout",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "caption-side",
                path: "/docs/caption-side",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Transitions & Animation",
        entries: &[
            DocsNavEntry {
                title: "transition-property",
                path: "/docs/transition-property",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "transition-behavior",
                path: "/docs/transition-behavior",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "transition-duration",
                path: "/docs/transition-duration",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "transition-timing-function",
                path: "/docs/transition-timing-function",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "transition-delay",
                path: "/docs/transition-delay",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "animation",
                path: "/docs/animation",
                page: Some(Page::Motion),
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Transforms",
        entries: &[
            DocsNavEntry {
                title: "backface-visibility",
                path: "/docs/backface-visibility",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "perspective",
                path: "/docs/perspective",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "perspective-origin",
                path: "/docs/perspective-origin",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "rotate",
                path: "/docs/rotate",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "scale",
                path: "/docs/scale",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "skew",
                path: "/docs/skew",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "transform",
                path: "/docs/transform",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "transform-origin",
                path: "/docs/transform-origin",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "transform-style",
                path: "/docs/transform-style",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "translate",
                path: "/docs/translate",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Interactivity",
        entries: &[
            DocsNavEntry {
                title: "accent-color",
                path: "/docs/accent-color",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "appearance",
                path: "/docs/appearance",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "caret-color",
                path: "/docs/caret-color",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "color-scheme",
                path: "/docs/color-scheme",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "cursor",
                path: "/docs/cursor",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "field-sizing",
                path: "/docs/field-sizing",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "pointer-events",
                path: "/docs/pointer-events",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "resize",
                path: "/docs/resize",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "scroll-behavior",
                path: "/docs/scroll-behavior",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "scroll-margin",
                path: "/docs/scroll-margin",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "scroll-padding",
                path: "/docs/scroll-padding",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "scroll-snap-align",
                path: "/docs/scroll-snap-align",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "scroll-snap-stop",
                path: "/docs/scroll-snap-stop",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "scroll-snap-type",
                path: "/docs/scroll-snap-type",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "touch-action",
                path: "/docs/touch-action",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "user-select",
                path: "/docs/user-select",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "will-change",
                path: "/docs/will-change",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "SVG",
        entries: &[
            DocsNavEntry {
                title: "fill",
                path: "/docs/fill",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "stroke",
                path: "/docs/stroke",
                page: None,
                children: &[],
            },
            DocsNavEntry {
                title: "stroke-width",
                path: "/docs/stroke-width",
                page: None,
                children: &[],
            },
        ],
    },
    DocsCategory {
        title: "Accessibility",
        entries: &[DocsNavEntry {
            title: "forced-color-adjust",
            path: "/docs/forced-color-adjust",
            page: None,
            children: &[],
        }],
    },
    DocsCategory {
        title: "Showcase",
        entries: &[
            DocsNavEntry {
                title: "Buttons",
                path: "/showcase/buttons",
                page: Some(Page::Buttons),
                children: &[],
            },
            DocsNavEntry {
                title: "Style Builder",
                path: "/showcase/style-builder",
                page: Some(Page::StyleBuilder),
                children: &[],
            },
            DocsNavEntry {
                title: "Kitchen Sink",
                path: "/showcase/kitchen-sink",
                page: Some(Page::KitchenSink),
                children: &[],
            },
            DocsNavEntry {
                title: "Real World: Translator",
                path: "/showcase/translator",
                page: Some(Page::Translator),
                children: &[],
            },
        ],
    },
];

pub fn docs_categories() -> &'static [DocsCategory] {
    DOCS_NAV_CATEGORIES
}

pub fn entry_for(page: Page) -> Option<&'static ShowcaseEntry> {
    SHOWCASE_ENTRIES.iter().find(|entry| entry.page == page)
}

pub fn toc_for(page: Page) -> &'static [TocEntry] {
    entry_for(page).map(|entry| entry.toc).unwrap_or(&[])
}

pub fn category_for(page: Page) -> Option<&'static str> {
    entry_for(page).map(|entry| entry.category)
}

fn find_path_in_entries(entries: &'static [DocsNavEntry], page: Page) -> Option<&'static str> {
    for entry in entries {
        if entry.page == Some(page) {
            return Some(entry.path);
        }
        if let Some(path) = find_path_in_entries(entry.children, page) {
            return Some(path);
        }
    }

    None
}

pub fn nav_path_for(page: Page) -> Option<&'static str> {
    for category in DOCS_NAV_CATEGORIES {
        if let Some(path) = find_path_in_entries(category.entries, page) {
            return Some(path);
        }
    }

    None
}

pub fn toc_offset(index: usize, total: usize) -> RelativeOffset {
    let y = if total <= 1 {
        0.0
    } else {
        index as f32 / (total.saturating_sub(1)) as f32
    };

    RelativeOffset { x: 0.0, y }
}

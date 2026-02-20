use iced::widget::operation::{self, RelativeOffset};
use iced::widget::{
    Id, Space, button, column, container, image, mouse_area, responsive, row, scrollable, stack,
    text,
};
use iced::{Alignment, Background, Element, Length, Size, Subscription, Task, Theme};
use std::time::Instant;
use twill::iced::{to_color, to_font_weight, to_padding};
use twill::tokens::{Color, FontWeight, Scale, Spacing};

mod components;
mod docs_index;
mod pages;
mod sidebar;
mod toc;

use components::TableOfContents;
use docs_index::{category_for, entry_for, nav_path_for, toc_for, toc_offset};
use pages::Page;
use sidebar::Sidebar;

const MOBILE_BREAKPOINT: f32 = 768.0;
const DESKTOP_BREAKPOINT: f32 = 1200.0;
const CONTENT_MAX_WIDTH: f32 = 920.0;

pub fn main() -> iced::Result {
    iced::application(Showcase::default, Showcase::update, Showcase::view)
        .title(Showcase::title)
        .theme(Showcase::theme)
        .subscription(Showcase::subscription)
        .run()
}

pub struct Showcase {
    sidebar: Sidebar,
    table_of_contents: TableOfContents,
    current_page: Page,
    is_dark: bool,
    mobile_nav_open: bool,
    content_scroll_id: Id,
    start_time: Instant,
    elapsed: f64,
    columns_preview_width: f32,
    aspect_preview_width: f32,
}

impl Default for Showcase {
    fn default() -> Self {
        Self {
            sidebar: Sidebar,
            table_of_contents: TableOfContents,
            current_page: Page::default(),
            is_dark: true,
            mobile_nav_open: false,
            content_scroll_id: Id::unique(),
            start_time: Instant::now(),
            elapsed: 0.0,
            columns_preview_width: 680.0,
            aspect_preview_width: 280.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PageSelected(Page),
    ToggleTheme,
    ToggleMobileNav,
    CloseMobileNav,
    TocJump(usize),
    NoOp,
    Tick(Instant),
    ColumnsPreviewWidthChanged(f32),
    AspectPreviewWidthChanged(f32),
}

impl Showcase {
    fn title(&self) -> String {
        String::from("Twill Iced Showcase")
    }

    fn snap_to_content(&self, offset: RelativeOffset) -> Task<Message> {
        operation::snap_to(self.content_scroll_id.clone(), offset)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PageSelected(page) => {
                self.current_page = page;
                self.mobile_nav_open = false;
                self.snap_to_content(RelativeOffset::START)
            }
            Message::ToggleTheme => {
                self.is_dark = !self.is_dark;
                Task::none()
            }
            Message::ToggleMobileNav => {
                self.mobile_nav_open = !self.mobile_nav_open;
                Task::none()
            }
            Message::CloseMobileNav => {
                self.mobile_nav_open = false;
                Task::none()
            }
            Message::TocJump(index) => {
                let toc = toc_for(self.current_page);
                if toc.is_empty() {
                    Task::none()
                } else {
                    let clamped_index = index.min(toc.len().saturating_sub(1));
                    let offset = toc_offset(clamped_index, toc.len());
                    self.snap_to_content(offset)
                }
            }
            Message::NoOp => Task::none(),
            Message::Tick(now) => {
                self.elapsed = now.duration_since(self.start_time).as_secs_f64();
                Task::none()
            }
            Message::ColumnsPreviewWidthChanged(width) => {
                self.columns_preview_width = width.clamp(280.0, 920.0);
                Task::none()
            }
            Message::AspectPreviewWidthChanged(width) => {
                self.aspect_preview_width = width.clamp(140.0, 520.0);
                Task::none()
            }
        }
    }

    fn twill_icon(&self, compact: bool) -> Element<'_, Message> {
        let px = if compact { 16.0 } else { 20.0 };
        let handle = image::Handle::from_bytes(include_bytes!("../../assets/icon.png").as_slice());

        image(handle)
            .width(Length::Fixed(px))
            .height(Length::Fixed(px))
            .into()
    }

    fn view_header(&self) -> Element<'_, Message> {
        let is_dark = self.is_dark;
        let theme_text = if is_dark { "Dark" } else { "Light" };
        let category = category_for(self.current_page).unwrap_or("General");
        let raw_nav_path = nav_path_for(self.current_page).unwrap_or("/showcase");
        let short_nav = raw_nav_path
            .trim_start_matches("/docs/")
            .trim_start_matches("/showcase/");
        let nav_segment = if short_nav.is_empty() {
            self.current_page.slug()
        } else {
            short_nav
        };
        let breadcrumb = format!("{category} / {nav_segment}");

        container(
            column![
                row![
                    self.twill_icon(true),
                    text("TWILL SHOWCASE").size(11).font(iced::Font {
                        weight: to_font_weight(FontWeight::SemiBold),
                        ..Default::default()
                    }),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                row![
                    text(self.current_page.docs_title())
                        .size(24)
                        .font(iced::Font {
                            weight: to_font_weight(FontWeight::Bold),
                            ..Default::default()
                        }),
                    Space::new().width(Length::Fill).height(Length::Shrink),
                    button(theme_text).on_press(Message::ToggleTheme),
                ]
                .align_y(Alignment::Center)
                .spacing(12),
                text(breadcrumb).size(12),
            ]
            .spacing(4),
        )
        .width(Length::Fill)
        .padding(to_padding(Spacing::S4))
        .style(move |_theme| container::Style {
            border: iced::Border {
                color: if is_dark {
                    to_color(Color::gray(Scale::S800))
                } else {
                    to_color(Color::gray(Scale::S300))
                },
                width: 1.0,
                ..Default::default()
            },
            background: Some(if is_dark {
                Background::Color(to_color(Color::gray(Scale::S900)))
            } else {
                Background::Color(to_color(Color::white()))
            }),
            ..container::Style::default()
        })
        .into()
    }

    fn view_page_intro(&self) -> Element<'_, Message> {
        if let Some(entry) = entry_for(self.current_page) {
            let category = category_for(self.current_page).unwrap_or("General");

            column![
                text(category.to_uppercase()).size(12).font(iced::Font {
                    weight: to_font_weight(FontWeight::SemiBold),
                    ..Default::default()
                }),
                text(entry.title).size(32).font(iced::Font {
                    weight: to_font_weight(FontWeight::Bold),
                    ..Default::default()
                }),
                text(entry.description).size(15),
            ]
            .spacing(5)
            .into()
        } else {
            Space::new()
                .width(Length::Shrink)
                .height(Length::Shrink)
                .into()
        }
    }

    fn view_page_scroll(&self) -> Element<'_, Message> {
        scrollable(
            column![
                self.view_page_intro(),
                self.current_page.view(
                    self.is_dark,
                    self.elapsed,
                    self.columns_preview_width,
                    self.aspect_preview_width,
                )
            ]
            .spacing(24),
        )
        .id(self.content_scroll_id.clone())
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn view_content_panel(&self, compact_toc: bool) -> Element<'_, Message> {
        let mut content = column![].spacing(12);

        if compact_toc {
            content = content.push(
                container(self.table_of_contents.view(self.current_page, true))
                    .max_width(CONTENT_MAX_WIDTH)
                    .center_x(Length::Fill),
            );
        }

        content = content.push(
            container(self.view_page_scroll())
                .max_width(CONTENT_MAX_WIDTH)
                .center_x(Length::Fill)
                .height(Length::Fill),
        );

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([24, 32])
            .into()
    }

    fn view_divider(&self) -> Element<'_, Message> {
        container(Space::new().width(Length::Fixed(1.0)).height(Length::Fill))
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(move |theme| {
                let is_dark = matches!(theme, iced::Theme::Dark);
                container::Style {
                    background: Some(Background::Color(if is_dark {
                        to_color(Color::gray(Scale::S800))
                    } else {
                        to_color(Color::gray(Scale::S300))
                    })),
                    ..container::Style::default()
                }
            })
            .into()
    }

    fn view_desktop(&self) -> Element<'_, Message> {
        row![
            self.sidebar.view(self.current_page, false),
            self.view_divider(),
            self.view_content_panel(false),
            self.view_divider(),
            container(self.table_of_contents.view(self.current_page, false))
                .padding([28, 16])
                .height(Length::Fill),
        ]
        .height(Length::Fill)
        .into()
    }

    fn view_tablet(&self) -> Element<'_, Message> {
        row![
            self.sidebar.view(self.current_page, false),
            self.view_divider(),
            self.view_content_panel(true),
        ]
        .height(Length::Fill)
        .into()
    }

    fn view_mobile_bar(&self) -> Element<'_, Message> {
        let mobile_nav_button = if self.mobile_nav_open {
            button("Close").on_press(Message::CloseMobileNav)
        } else {
            button("Menu").on_press(Message::ToggleMobileNav)
        };

        container(
            row![
                mobile_nav_button,
                text(self.current_page.docs_title()).size(14),
                Space::new().width(Length::Fill).height(Length::Shrink),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .width(Length::Fill)
        .padding(to_padding(Spacing::S3))
        .style(move |theme| {
            let is_dark = matches!(theme, iced::Theme::Dark);
            container::Style {
                border: iced::Border {
                    color: if is_dark {
                        to_color(Color::gray(Scale::S800))
                    } else {
                        to_color(Color::gray(Scale::S300))
                    },
                    width: 1.0,
                    ..Default::default()
                },
                background: Some(if is_dark {
                    Background::Color(to_color(Color::gray(Scale::S900)))
                } else {
                    Background::Color(to_color(Color::white()))
                }),
                ..container::Style::default()
            }
        })
        .into()
    }

    fn view_mobile_drawer(&self) -> Element<'_, Message> {
        let backdrop = container(
            container(Space::new().width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(iced::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.35,
                    })),
                    ..container::Style::default()
                }),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let drawer_panel = container(self.sidebar.view(self.current_page, true))
            .width(Length::Fixed(304.0))
            .height(Length::Fill)
            .style(move |theme| {
                let is_dark = matches!(theme, iced::Theme::Dark);
                container::Style {
                    border: iced::Border {
                        color: if is_dark {
                            to_color(Color::gray(Scale::S800))
                        } else {
                            to_color(Color::gray(Scale::S300))
                        },
                        width: 1.0,
                        ..Default::default()
                    },
                    ..container::Style::default()
                }
            });

        let close_area = mouse_area(
            container(Space::new().width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .on_press(Message::CloseMobileNav);

        let drawer = row![drawer_panel, close_area,]
            .width(Length::Fill)
            .height(Length::Fill);

        stack([backdrop.into(), drawer.into()])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_mobile(&self) -> Element<'_, Message> {
        let base = column![self.view_mobile_bar(), self.view_content_panel(true)]
            .spacing(8)
            .height(Length::Fill);

        if self.mobile_nav_open {
            stack([base.into(), self.view_mobile_drawer()])
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            base.into()
        }
    }

    fn view_docs_layout(&self, size: Size) -> Element<'_, Message> {
        if size.width >= DESKTOP_BREAKPOINT {
            self.view_desktop()
        } else if size.width >= MOBILE_BREAKPOINT {
            self.view_tablet()
        } else {
            self.view_mobile()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let docs_layout = responsive(|size| self.view_docs_layout(size));

        column![self.view_header(), docs_layout]
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.current_page == Page::Motion {
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::Tick(Instant::now()))
        } else {
            Subscription::none()
        }
    }
}

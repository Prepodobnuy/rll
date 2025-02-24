use std::io::stdout;

use crossterm::{
    cursor::{
        Hide, 
        MoveTo, 
        Show
    }, 
    execute, 
    style::Print, 
    terminal::{
        self, 
        disable_raw_mode, 
        enable_raw_mode, 
        Clear, 
        ClearType, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }, 
    ExecutableCommand,
};

use crate::{
    backends::traits::DrawerTrair, layout::{
        Container, 
        StyleLink
    }, style::{
        Align, ContentWrap, Orientation, Size, Style 
    }
};

use super::colors::remove_colors_from_len;

pub struct Drawer {
    render_parts: Vec<RenderPart>,
}

impl Drawer {
    pub fn new() -> Self {
        Drawer {
            render_parts: vec![],
        }
    }
}

impl DrawerTrair for Drawer {
    fn render(&mut self, main_container: Container, style_links: Vec<StyleLink>) {
        let geometry = terminal::size().unwrap();

        self.render_parts = prepare_render_parts(
            main_container, 
            style_links, 
            (
                geometry.0 as u32, 
                geometry.1 as u32,
                0,
                0
            ),
        );

        let mut stdout = stdout();
        let _ = stdout.execute(Clear(ClearType::All));
    }

    fn display(& self) {
        for part in &self.render_parts {
            part.display();
        }
    }
}

struct RenderPart {
    geometry: (u32, u32, u32, u32),
    wrap_contaiment: bool,
    h_align: Align,
    v_align: Align,
    contaiment: String,
}

impl RenderPart {
    fn new(
        geometry: (u32, u32, u32, u32),
        wrap_contaiment: bool,
        h_align: Align,
        v_align: Align,
        contaiment: String,
    ) -> Self {
        RenderPart {
            geometry,
            wrap_contaiment,
            h_align,
            v_align,
            contaiment,
        }
    }

    fn from_container(geometry: (u32, u32, u32, u32), container: &Container, styles: &Vec<Style>) -> Self {
        let mut margin = (0, 0, 0, 0);
        let mut wrap_contaiment = false;
        let mut h_align = Align::Left;
        let mut v_align = Align::Top;
        let contaiment = container.contaiment.clone();

        for style in styles {
            match style {
                Style::ContentWrap(content_wrap) => match content_wrap {
                    ContentWrap::Wrap => wrap_contaiment = true,
                    ContentWrap::NoWrap => wrap_contaiment = false,
                },
                Style::HAlign(align) => h_align = align.clone(),
                Style::VAlign(align) => v_align = align.clone(),
                _ => {},
            }
        }

        RenderPart {
            geometry,
            wrap_contaiment,
            h_align,
            v_align,
            contaiment,
        }
    }

    fn display(& self) {
        let mut contaiment = self.contaiment.clone();
        let mut contaiment_len = contaiment.len();
        contaiment_len = remove_colors_from_len(contaiment_len, &contaiment);
        if contaiment_len == 0 {return;}

        let mut h_pos = 0;
        let mut v_pos = 0;

        h_pos = match self.h_align {
            Align::Left => 0,
            Align::Right => self.geometry.0 - contaiment_len as u32,
            Align::Center => (self.geometry.0 - contaiment_len as u32) / 2,
            _ => h_pos,
        };

        v_pos = match self.v_align {
            Align::Top => 0,
            Align::Center => self.geometry.1 / 2,
            Align::Bottom => self.geometry.1 - 1,
            _ => v_pos,
        };

        let mut stdout = stdout();

        let _ = stdout.execute(Hide);

        if self.wrap_contaiment {
            for char in contaiment.chars() {
                let _ = stdout.execute(MoveTo(h_pos as u16 + self.geometry.2 as u16, v_pos as u16 + self.geometry.3 as u16));
                let _ = stdout.execute(Print(char));
                if h_pos == self.geometry.0 {
                    h_pos = 0;
                    v_pos += 1;
                }
                if v_pos > self.geometry.1 {
                    break;
                }
            }
        } else {
            let _ = stdout.execute(MoveTo(h_pos as u16 + self.geometry.2 as u16, v_pos as u16 + self.geometry.3 as u16));
            let size: usize = (self.geometry.0 - h_pos - self.geometry.2) as usize;
            //let _ = stdout.execute(
            //    Print(
            //        &contaiment.clone()[
            //            0
            //            ..
            //            if size > 1 {size} else {1}
            //        ]
            //    ));
        }

    }
}

fn prepare_render_parts(
    container: Container, 
    style_links: Vec<StyleLink>,
    mut geometry: (u32, u32, u32, u32),
) -> Vec<RenderPart> {
    let mut result: Vec<RenderPart> = Vec::new();

    let styles = find_container_style(&style_links, &container);

    let mut horizontal = true;

    for style in &styles {match style {
        Style::Orientation(orientation) => match orientation {
            Orientation::Horizontal => horizontal=true,
            Orientation::Vertical => horizontal=false,
        },
        _ => {},
    }}

    for style in &styles {match style {
        Style::Margin(size, size1, size2, size3) => {
            let margin_left = match size {
                Size::Percent(s) => find_real_size(*s, geometry.0),
                Size::Fixed(s) => if *s > geometry.0 {geometry.0 - 1} else {*s},
            };
            let margin_top = match size1 {
                Size::Percent(s) => find_real_size(*s, geometry.1),
                Size::Fixed(s) => if *s > geometry.1 {geometry.1 - 1} else {*s},
            };
            let margin_right = match size2 {
                Size::Percent(s) => find_real_size(*s, geometry.0),
                Size::Fixed(s) => if *s > geometry.0 {geometry.0 - 1} else {*s},
            };
            let margin_bottom = match size3 {
                Size::Percent(s) => find_real_size(*s, geometry.1),
                Size::Fixed(s) => if *s > geometry.1 {geometry.1 - 1} else {*s},
            };


            geometry.0 -= margin_left;
            geometry.2 += margin_left;

            geometry.1 -= margin_top;
            geometry.3 += margin_top;

            geometry.0 = if geometry.0 - margin_right > 1 {geometry.0 - margin_right} else {1};
            geometry.1 = if geometry.1 - margin_bottom > 1 {geometry.1 - margin_bottom} else {1};
            
            break;
        },
        _ => {},
    }}

    result.push(RenderPart::from_container(geometry, &container, &styles));

    let childs = container.childs();
    let child_sizes = find_childs_size(
        &style_links, 
        &childs, 
        if horizontal {geometry.0} else {geometry.1},
    );

    for (i, c) in childs.iter().enumerate() {
        result.extend(prepare_render_parts(
            c.clone(), 
            style_links.clone(), 
            (
                if horizontal {child_sizes[i]} else {geometry.0},
                if !horizontal {child_sizes[i]} else {geometry.1},
                if !horizontal {geometry.2} else {
                    let mut sum = geometry.2;
                    child_sizes.iter().enumerate().for_each(|(j, f)| {
                        if j < i {
                            sum += f 
                        }
                    });
                    sum
                },
                if horizontal {geometry.3} else {
                    let mut sum = geometry.3;
                    child_sizes.iter().enumerate().for_each(|(j, f)| {
                        if j < i {
                            sum += f
                        }
                    });
                    sum
                },
            )
        ));
    }

    result
}

fn find_childs_size(style_links: &Vec<StyleLink>, childs: &Vec<Container>, parent_size: u32) -> Vec<u32> {
    let sizes: Vec<u32> = childs.iter()
        .map(|c| {
            let styles = find_container_style(style_links, c);
            let mut min_size = Size::Fixed(1);
            let mut max_size = Size::Percent(100);

            for style in styles {
                match style {
                    Style::MinSize(v) => {min_size = v},
                    Style::MaxSize(v) => {max_size = v},
                    _ => {},
                }
            }

            let _max = match max_size {
                Size::Fixed(v) => {
                    if parent_size < v {parent_size} else {v}
                },
                Size::Percent(v) => {
                    find_real_size(v, parent_size)
                }
            };

            let _min = match min_size {
                Size::Fixed(v) => {
                    if parent_size < v {parent_size} else {v}
                },
                Size::Percent(v) => {
                    find_real_size(v, parent_size)
                }
            };

            if _min > _max {_min} else {_max}
        }).collect();

    let sum: u32 = sizes.iter().sum();
    let difference = sum as f32 / parent_size as f32;
    
    sizes.iter()
        .map(|s| {
            (*s as f32 / difference).round() as u32
        })
        .collect()
}

fn find_real_size(percent_size: u8, max_size: u32) -> u32 {
    let percent_size = percent_size.clamp(1, 100);
    ((percent_size as u32 * max_size) / 100).clamp(1, u32::max_value())
}

fn find_container_style(style_links: &Vec<StyleLink>, container: &Container) -> Vec<Style> {
    style_links
        .into_iter()
        .filter(|l| {
            l.id.as_ref().map_or(false, |id| id.to_string() == container.id)
                || l.class.as_ref().map_or(false, |class| container.classes.contains(class))
        })
        .flat_map(|l| l.style.clone())
        .collect()
}

//fn collect_render_parts(container: Container, style_links: Vec<StyleLink>, w: u32, h: u32, x: u32, y: u32, horizontal: bool) -> Vec<RenderPart> {
//    let mut result: Vec<RenderPart> = Vec::new();
//
//    let mut orientation = Orientation::Horizontal;
//    let mut wrap_contaiment = ContentWrap::NoWrap;
//    let mut h_align = Align::Left;
//    let mut v_align = Align::Top;
//    let mut _w = w;
//    let mut _h = h;
//    let mut _x = x;
//    let mut _y = y;
//    let size = if horizontal {w} else {h}; 
//    let mut min_size: u32 = 1;
//
//    let styles = find_container_style(style_links.clone(), &container);
//    let childs = container.childs();
//    
//    for style in &styles {
//        match style {
//            Style::Orientation(v) => {orientation = v.clone()},
//            Style::ContentWrap(v) => {wrap_contaiment = v.clone()},
//            Style::HAlign(v) => {h_align = v.clone()},
//            Style::VAlign(v) => {v_align = v.clone()},
//            _ => {},
//        }
//    }
//
//    if childs.is_empty() {
//        let mut _contaiment = container.contaiment;
//
//        result.push(RenderPart::new(
//            x,
//            y,
//            w,
//            h,
//            wrap_contaiment,
//            h_align,
//            v_align,
//            _contaiment,
//        ));
//    } else {
//        let size_params = collect_childs_size_params(style_links.clone(), &childs, horizontal, size);
//        for (i, child) in childs.iter().enumerate() {
//            let styles = find_container_style(
//                style_links.clone(), 
//                child, 
//            );
//
//            for style in styles {
//                match style {
//                    Style::Orientation(v) => {orientation = v}
//                    _ => {},
//                }
//            }
//
//            result.extend(collect_render_parts(
//                child.clone(), 
//                style_links.clone(), 
//                if !horizontal {w} else {size_params[i]}, 
//                if horizontal {h} else {size_params[i]}, 
//                if !horizontal {x} else {
//                    let mut sum = x;
//                    size_params.iter().enumerate().for_each(|(j, f)| {
//                        if j < i {
//                            sum += f 
//                        }
//                    });
//                    sum
//                },
//                if horizontal {y} else {
//                    let mut sum = y;
//                    size_params.iter().enumerate().for_each(|(j, f)| {
//                        if j < i {
//                            sum += f
//                        }
//                    });
//                    sum
//                },
//                match orientation {
//                    Orientation::Horizontal => true,
//                    Orientation::Vertical => false,
//                },
//            ));
//        }
//    };
//
//    result
//}
//
//
//
//
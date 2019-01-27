use specs::prelude::*;

use std::collections::VecDeque;

pub struct DisplayText {
    pub x: i32,
    pub y: i32,
    pub th: i32,
    pub text: String,
    pub color: raylib::Color,
}

#[derive(Clone, Debug)]
pub enum DialogSeg {
    DString(DialogString),
}

impl DialogSeg {
    fn duration(&self) -> f32 {
        match self {
            _ => 0.01,
        }
    }

    fn display_text(&self, x: i32, y: i32, th: i32, t: f32) -> DisplayText {
        match self {
            DialogSeg::DString(ref s) => DisplayText {
                x,
                y,
                th,
                text: s.text.clone(),
                color: s.color,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DialogString {
    pub text: String,
    pub color: raylib::Color,
}

#[derive(Component)]
pub struct Dialog {
    pub time: f32,
    pub thumbnail: DialogThumbnail,
    pub line: Vec<DisplayText>,
    pub current: Option<DialogSeg>,
    pub say: VecDeque<DialogSeg>,
    should_clear: bool,
    is_done: bool,
}

pub struct DialogCharSeq(pub VecDeque<DialogSeg>);

impl<'a> From<std::str::Chars<'a>> for DialogCharSeq {
    fn from(chars: std::str::Chars<'a>) -> DialogCharSeq {
        let queue: VecDeque<_> = chars
            .map(|c| {
                DialogSeg::DString(DialogString {
                    text: c.to_string(),
                    color: raylib::Color::BLACK,
                })
            })
            .collect();
        DialogCharSeq(queue)
    }
}

impl<'a> From<&'a str> for DialogCharSeq {
    fn from(s: &str) -> DialogCharSeq {
        let mut queue = VecDeque::new();
        let space = DialogSeg::DString(DialogString {
            text: "  ".to_owned(),
            color: raylib::Color::BLACK,
        });
        for word in s.split_whitespace().map(|word| {
            DialogSeg::DString(DialogString {
                text: word.to_owned(),
                color: raylib::Color::BLACK,
            })
        }) {
            queue.push_back(word);
            queue.push_back(space.clone())
        }

        DialogCharSeq(queue)
    }
}

impl Dialog {
    pub fn new(thumbnail: DialogThumbnail, say: VecDeque<DialogSeg>) -> Self {
        Dialog {
            time: 0.0,
            thumbnail,
            line: Vec::new(),
            current: None,
            say,
            should_clear: false,
            is_done: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn should_clear(&self) -> bool {
        self.should_clear
    }

    pub fn clear(&mut self) {
        self.line = Vec::new();
    }

    pub fn draw_text(&mut self, ctx: &mut raylib::DrawContext, text_box: &raylib::Rectangle) {
        // set up cursors
        let mut cx = text_box.x;
        let mut cy = text_box.y;

        let th = 40.0;
        // let text_width = 30;
        // let TH = 20.0;
        self.time += ctx.app_context.get_frame_time();

        for dt in self.line.iter() {
            let tw = raylib::measure_text(&dt.text, dt.th);
            if cx + tw as f32 > text_box.width {
                cx = text_box.x;
                cy += th;
            }
            // Quit if no more lines
            if cy + th > text_box.y + text_box.height {
                self.should_clear = true;
                return;
            }
            // draw the text
            ctx.draw_text(&dt.text, cx as i32, cy as i32, dt.th as i32, &dt.color);
            cx += tw as f32;
        }

        self.current = match self.current.take() {
            Some(seg) => {
                // attempt to draw the text
                let dur = seg.duration();
                let t = self.time / dur;
                let dt = seg.display_text(cx as i32, cy as i32, th as i32, t);
                let tw = raylib::measure_text(&dt.text, dt.th);
                if cx + tw as f32 > text_box.width {
                    cx = text_box.x;
                    cy += th;
                }
                if cy + th > text_box.y + text_box.height {
                    self.should_clear = true;
                    // clear the time
                    self.time = 0.0;
                    Some(seg)
                } else {
                    // draw the text
                    ctx.draw_text(&dt.text, cx as i32, cy as i32, dt.th as i32, &dt.color);
                    // done with the text
                    if t > 1.0 {
                        self.line.push(dt);
                        None
                    } else {
                        Some(seg)
                    }
                }
            }
            None => match self.say.pop_front() {
                None => {
                    self.is_done = true;
                    None
                }
                Some(seg) => {
                    self.time = 0.0;
                    Some(seg)
                }
            },
        }
    }
}

pub struct DialogThumbnail {
    pub thumbnail: DialogImage,
}

impl DialogThumbnail {
    pub fn new(thumbnail: DialogImage) -> Self {
        DialogThumbnail { thumbnail }
    }

    pub fn draw(&self, ctx: &mut raylib::DrawContext, rect: &raylib::Rectangle) {
        match self.thumbnail {
            DialogImage::Player => {
                let (ml, mb, mr, mt) = (5.0, 5.0, 5.0, 5.0);
                let pm = 30.0;
                let pw = rect.height.min(rect.width) - pm;

                // Draw the Background
                ctx.draw_rectangle(
                    rect.x as i32,
                    rect.y as i32,
                    rect.width as i32,
                    rect.height as i32,
                    &raylib::Color::BLACK,
                );
                // Draw the outline
                let outline_rect = raylib::Rectangle::new(
                    rect.x + ml,
                    rect.y + mt,
                    rect.width - ml - mr,
                    rect.height - mt - mb,
                );
                ctx.draw_rectangle_lines_ex(&outline_rect, 2, &raylib::Color::WHITE);

                // draw the player
                ctx.draw_rectangle(
                    (rect.x + pm) as i32,
                    (rect.y + pm - mt) as i32,
                    pw as i32,
                    pw as i32,
                    &raylib::Color::WHITE,
                );

                // draw face
                draw_face(
                    ctx,
                    &raylib::Vector2::new(rect.x + pm + pw / 2.0, rect.y + pm + 20.0),
                    10.0,
                    &raylib::Color::BLACK,
                )
            }
        }
    }
}

pub enum DialogImage {
    Player,
}

pub fn draw_face(
    ctx: &mut raylib::DrawContext,
    pos: &raylib::Vector2,
    radius: f32,
    color: &raylib::Color,
) {
    // negate because y increases downward
    let r_eye = -radius * raylib::Vector2::from_angle(3.0 * std::f32::consts::PI / 4.0) + *pos;
    let l_eye = -radius * raylib::Vector2::from_angle(1.0 * std::f32::consts::PI / 4.0) + *pos;
    let mouth = -radius * raylib::Vector2::from_angle(3.0 * std::f32::consts::PI / 2.0) + *pos;

    ctx.draw_circle_v(&l_eye, 5.0, color);
    ctx.draw_circle_v(&r_eye, 5.0, color);
    ctx.draw_circle_v(&mouth, 5.0, color);
}

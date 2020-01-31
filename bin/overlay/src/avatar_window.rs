use crate::{
    backend::{Backend, BackendError, BackendTexture, BackendWindow, WindowRef},
    bridge::{Avatar, Char},
    image_data::ImageData,
    Rect,
};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};

pub struct AvatarWindow<B: Backend> {
    inner: WindowRef<B>,
    pub char_texture: Option<(Char, Rc<BackendTexture<B>>)>,
    last_frame: u64,
    hidden: bool,
    pos: Option<(i32, i32)>,
    hide_until: Option<Instant>,
    size: u16,
}

impl<B: Backend> AvatarWindow<B> {
    pub fn texture(&self) -> Option<Rc<BackendTexture<B>>> {
        self.char_texture
            .as_ref()
            .map(|char_texture| Rc::clone(&char_texture.1))
    }
    pub fn new(inner: WindowRef<B>, size: u16) -> Self {
        AvatarWindow {
            inner,
            char_texture: None,
            last_frame: 0,
            hidden: true,
            pos: None,
            hide_until: None,
            size,
        }
    }
    pub fn maintain(&mut self, frame: u64, updated: bool) -> bool {
        if updated {
            if self.last_frame + 240 * 30 < frame {
                //println!("delete");
                return false;
            } else if self.last_frame != frame {
                //println!("hide");
                self.hide();
            }
        } else {
            if let Some(hide_until) = self.hide_until {
                //println!("hide until");
                let now = Instant::now();
                if now > hide_until {
                    //println!("expired");
                    self.hide_until = None;
                    if !self.hidden {
                        self.inner.borrow_mut().show();
                        self.draw();
                    }
                }
            }
        }
        true
    }
    pub fn update(
        &mut self,
        avatar: &Avatar,
        image: &mut ImageData,
        rect: &Rect,
        frame: u64,
        mut redraw: bool,
        size: u16,
    ) -> bool {
        if let Err(err) = self.update_char(avatar.char, image) {
            eprintln!("Update char window: {:?}", err);
            self.hide();
            return false;
        }
        if size != self.size {
            self.size = size;
            self.inner.borrow_mut().set_size(size as u32, size as u32);
            redraw = true;
        }
        let size = self.size as i32;
        let x = avatar.pos.x - size / 2;
        let y = avatar.pos.y - size - 16;
        let mut appeared = false;
        if (rect.width as i32 - size > x && x > 0) && (rect.height as i32 - size > y && y > 0) {
            self.set_position(rect.x + x, rect.y + y);
            if self.show() || redraw {
                appeared = true;
                self.draw();
            }
        } else {
            //if characters are visible, but out of screen - don't show them. but mark avatar window as used
            self.hide();
        }
        self.last_frame = frame;
        appeared
    }
    fn update_char(&mut self, char: Char, image: &mut ImageData) -> Result<(), BackendError<B>> {
        if let Some((old_char, _)) = &self.char_texture {
            if *old_char == char {
                return Ok(());
            } else {
                let (_, old_texture) = self.char_texture.take().unwrap();
                //self.inner.drop_texture(old_texture);
            }
        }
        let texture = self.inner.borrow_mut().create_texture(image)?;
        self.char_texture = Some((char, Rc::new(texture)));
        Ok(())
    }
    fn draw(&mut self) -> Result<(), BackendError<B>> {
        if let Some((_, texture)) = &self.char_texture {
            let size = self.size as u32;
            let src = Rect::new(0, 0, 128, 128);
            let dst = Rect::new(0, 0, size, size);
            self.inner.borrow_mut().draw_texture(texture, &src, &dst)?;
        }
        Ok(())
    }
    fn show(&mut self) -> bool {
        if self.hidden {
            if let Some(hide_until) = self.hide_until {
                let now = Instant::now();
                if now > hide_until {
                    self.hide_until = None;
                } else {
                    self.hidden = false;
                    return false;
                }
            }
            self.hidden = false;
            self.inner.borrow_mut().show();
            true
        } else {
            false
        }
    }
    fn hide(&mut self) {
        if !self.hidden {
            self.hidden = true;
            self.inner.borrow_mut().hide();
        }
    }
    fn set_position(&mut self, x: i32, y: i32) {
        if self.pos != Some((x, y)) {
            self.pos = Some((x, y));
        } else {
            return;
        }
        self.inner.borrow_mut().set_position(x, y);
    }
    pub fn hide_for_ms(&mut self, delay: u32) {
        self.hide_until = Some(Instant::now() + Duration::from_millis(delay as u64));
        self.inner.borrow_mut().hide();
    }
}

impl<B: Backend> crate::windowing::OverlayWindow<B> for AvatarWindow<B> {
    fn backend_window(&self) -> &WindowRef<B> {
        &self.inner
    }
}

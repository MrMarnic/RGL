use crate::engine::game_engine::GameEngine;
use crate::ui::bounds::Bounds;

pub struct Animator {
    pub settings: AnimationSettings
}

impl Animator {
    pub fn new() -> Animator {
        Animator { settings: AnimationSettings::new() }
    }

    pub fn is_changing(&self,attr:AnimationAttribute) -> bool{
        return match attr {
            AnimationAttribute::COLOR => {
                self.settings.r.is_some() | self.settings.g.is_some() | self.settings.b.is_some() | self.settings.a.is_some()
            },
            AnimationAttribute::POS => {
                self.settings.x.is_some() | self.settings.y.is_some()
            },
            AnimationAttribute::SIZE => {
                self.settings.width.is_some() | self.settings.height.is_some()
            }
        }
    }

    pub fn update(&mut self,engine:&mut GameEngine) {
        if let Some(x) = self.settings.x.as_mut() {
            if x.disabled {
                self.settings.x = None;
            } else {
                x.update(engine.delta_time);
            }
        }
        if let Some(x) = self.settings.y.as_mut() {
            if x.disabled {
                self.settings.y = None;
            } else {
                x.update(engine.delta_time);
            }
        }
        if let Some(x) = self.settings.width.as_mut() {
            if x.disabled {
                self.settings.width = None;
            } else {
                x.update(engine.delta_time);
            }
        }
        if let Some(x) = self.settings.height.as_mut() {
            if x.disabled {
                self.settings.height = None;
            } else {
                x.update(engine.delta_time);
            }
        }
        if let Some(x) = self.settings.r.as_mut() {
            if x.disabled {
                self.settings.r = None;
            } else {
                x.update(engine.delta_time);
            }
        }
        if let Some(x) = self.settings.g.as_mut() {
            if x.disabled {
                self.settings.g = None;
            } else {
                x.update(engine.delta_time);
            }
        }
        if let Some(x) = self.settings.b.as_mut() {
            if x.disabled {
                self.settings.b = None;
            } else {
                x.update(engine.delta_time);
            }
        }
        if let Some(x) = self.settings.a.as_mut() {
            if x.disabled {
                self.settings.a = None;
            } else {
                x.update(engine.delta_time);
            }
        }
    }

    pub fn apply(&self,x:&mut f32,y:&mut f32,width:&mut f32,height:&mut f32,r:&mut f32,g:&mut f32,b:&mut f32,a:&mut f32) {
        if let Some(xa) = self.settings.x.as_ref() {
            xa.apply(x);
        }
        if let Some(xa) = self.settings.y.as_ref() {
            xa.apply(y);
        }
        if let Some(xa) = self.settings.width.as_ref() {
            xa.apply(width);
        }
        if let Some(xa) = self.settings.height.as_ref() {
            xa.apply(height);
        }
        if let Some(xa) = self.settings.r.as_ref() {
            xa.apply(r);
        }
        if let Some(xa) = self.settings.g.as_ref() {
            xa.apply(g);
        }
        if let Some(xa) = self.settings.b.as_ref() {
            xa.apply(b);
        }
        if let Some(xa) = self.settings.a.as_ref() {
            xa.apply(a);
        }
    }
}

pub struct AnimationSettings {
    pub x: Option<AnimationDirection>,
    pub y: Option<AnimationDirection>,
    pub width: Option<AnimationDirection>,
    pub height: Option<AnimationDirection>,
    pub r: Option<AnimationDirection>,
    pub g: Option<AnimationDirection>,
    pub b: Option<AnimationDirection>,
    pub a: Option<AnimationDirection>
}

impl AnimationSettings {
    pub fn new() -> AnimationSettings {
        return AnimationSettings {
            x: None,
            y: None,
            width: None,
            height: None,
            r: None,
            g: None,
            b: None,
            a: None
        }
    }
}

pub struct AnimationDirection {
    pub dis_per_sec:f32,
    pub target:f32,
    pub current:f32,
    pub change:f32,
    pub disabled:bool,
    pub finished:bool,
    pub to:bool,
    pub original:f32,
    pub revert: bool,
    pub delay:f32,
    pub running: f32
}

impl AnimationDirection {
    pub fn to(now:f32,to:f32,time:f32) -> AnimationDirection{
        AnimationDirection {
            dis_per_sec: (to-now) / time,
            target: to,
            current: now,
            change: 0.0,
            disabled: false,
            finished: false,
            to: true,
            original: now,
            revert: false,
            delay: 0.0,
            running: 0.0
        }
    }

    pub fn dis(now:f32,dis:f32,time:f32) -> AnimationDirection{
        AnimationDirection {
            dis_per_sec: dis/time,
            target: dis,
            current: 0.0,
            change: 0.0,
            disabled: false,
            finished: false,
            to: false,
            original: now,
            revert: false,
            delay: 0.0,
            running: 0.0
        }
    }

    pub fn revert(mut self) -> Self {
        self.revert = true;
        self
    }

    pub fn delay(mut self, delay:f32) -> Self {
        self.delay = delay;
        self
    }

    pub fn running(mut self, running:f32) -> Self {
        self.running = running;
        self
    }

    pub fn update(&mut self,delta:f32) {
        if self.delay <= 0.0 {
            if !self.finished {
                self.current += self.dis_per_sec * delta;
                self.change = self.dis_per_sec * delta;

                if self.dis_per_sec > 0.0 {
                    if self.current >= self.target {
                        self.current = self.original;
                        self.finished = true;
                        if self.running == 0.0 {
                            self.disabled = true;
                        }
                    }
                } else {
                    if self.current <= self.target {
                        self.current = self.target;
                        self.finished = true;
                        if self.running == 0.0 {
                            self.disabled = true;
                        }
                    }
                }
            } else {
                self.running -= delta;

                if self.running <= 0.0 {
                    self.disabled = true;
                }
            }
        } else {
            self.delay -= delta;
        }
    }

    fn apply(&self,v:&mut f32) {
        if self.delay <= 0.0{
            if !self.finished {
                if self.to {
                    *v = self.current;
                } else {
                    *v += self.change;
                }
            } else {
                if self.revert {
                    *v = self.original;
                } else {
                    if self.to {
                        *v = self.target;
                    } else {
                        *v += self.change;
                    }
                }
            }
        }
    }
}

pub enum AnimationAttribute {
    COLOR,
    POS,
    SIZE
}
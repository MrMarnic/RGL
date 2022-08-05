/*
use std::collections::HashMap;
use crate::text::character::Character;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use crate::objects::vertex_buffer::VertexBuffer;
use wgpu::Device;
use std::rc::Rc;
use crate::objects::texture_object::TextureObject;

pub struct Font {
    pub info:InfoData,
    pub common:CommonData,
    pub texture: Rc<TextureObject>,
    pub char_count:i32,
    pub characters:HashMap<char,Character>
}

impl Font {
    pub fn new(working_dir:String,path:String,texture:Rc<TextureObject>) -> Font {
        let file = File::open(format!("{}\\{}",working_dir,path)).unwrap();

        let mut reader = BufReader::new(file);

        let mut info = String::new();
        reader.read_line(&mut info).unwrap();

        let info_data = InfoData::from_string(info.trim());

        let mut common = String::new();
        reader.read_line(&mut common).unwrap();

        let common_data = CommonData::from_string(common.trim());

        reader.read_line(&mut String::new());
        let mut test = String::new();
        reader.read_line(&mut test).unwrap();
        let char_count = Font::read_count(&mut test.trim());
        let mut chars = HashMap::new();

        for i in 0..char_count {
            let mut line = String::new();
            reader.read_line(&mut line);
            line.trim();
            let mut c = Font::read_character(&mut line);
            c.set_coord_and_vertecies_and_indecies(texture.clone());
            chars.insert(c.id as char,c);
        }

        return Font {
            info: info_data,
            common: common_data,
            texture,
            char_count,
            characters: chars
        }
    }

    fn read_count(line:&str) -> i32 {
        return i32::from_str(line.split(" ").collect::<Vec<&str>>()[1].split("=").collect::<Vec<&str>>()[1]).unwrap();
    }

    fn read_character(line:&str) -> Character {
        let mut character = Character::empty();
        let attribs = line.split_whitespace().collect::<Vec<&str>>();
        for value in attribs {
            if value.starts_with("char") {
                continue;
            }  else {
                let value= value.split("=").collect::<Vec<&str>>();

                if value[0] == "id" {
                    character.id = u8::from_str(value[1]).unwrap();
                }else if value[0] == "x" {
                    character.x = i32::from_str(value[1]).unwrap();
                }else if value[0] == "y" {
                    character.y = i32::from_str(value[1]).unwrap();
                }else if value[0] == "width" {
                    character.width = i32::from_str(value[1]).unwrap();
                }else if value[0] == "height" {
                    character.height = i32::from_str(value[1]).unwrap();
                }else if value[0] == "xoffset" {
                    character.x_offset = f32::from_str(value[1]).unwrap();
                }else if value[0] == "yoffset" {
                    character.y_offset = f32::from_str(value[1]).unwrap();
                }else if value[0] == "xadvance" {
                    character.x_advance = i32::from_str(value[1]).unwrap();
                }else if value[0] == "page" {
                    character.page = i32::from_str(value[1]).unwrap();
                }else if value[0] == "chnl" {
                    character.chnl = i32::from_str(value[1]).unwrap();
                }
            }
        };

        return character;
    }

    pub fn create_vertex_buffer(&self,text:String,device:&Device) -> VertexBuffer {
        let mut vertecies = vec![];
        let mut indecies = vec![];

        for c in text.chars() {
            if !c.is_whitespace() {
                let char = &self.characters[&c];
            }
        }

        return VertexBuffer::new(device,vertecies,indecies,false);
    }

    pub fn get_width(&self, text:&String) -> f32 {
        let mut last_pos = 0.0;

        for (id,c) in text.chars().enumerate() {
            if !c.is_whitespace() {
                let char = &self.characters[&c];
                last_pos += char.x_advance as f32;
            } else {
                last_pos += 10.0;
            }
        }

        return last_pos;
    }

    pub fn get_width_char(&self, c:&char) -> f32 {
        let char = &self.characters[&c];
        return char.x_advance as f32;
    }

    pub fn get_suiting(&self, mut text:String, width:i32) -> (String,String) {
        let mut n = String::new();

        while self.get_width(&n) <= width as f32 && !text.is_empty(){
            let c = text.remove(0);

            if self.get_width(&n) + self.get_width_char(&c) <= width as f32{
                n.push(c);
            } else {
                text.insert(0,c);
                return (n,text);
            }
        }

        return (n,text);
    }

    pub fn get_height(&self, text:&String) -> f32 {
        let mut last_pos = 0.0;
        let mut count = 0;

        for (id,c) in text.chars().enumerate() {
            if !c.is_whitespace() {
                let char = &self.characters[&c];
                last_pos += char.height as f32;
                count+=1;
            }
        }

        return last_pos / count as f32;
    }
}

pub struct InfoData {
    pub name:String,
    pub size:i32,
    pub bold:i32,
    pub italic:i32,
    pub charset:String,
    pub unicode:i32,
    pub stretchH:i32,
    pub smooth:i32,
    pub aa:i32
}

impl InfoData {
    pub fn from_string(line:&str) -> InfoData {
        let mut info = InfoData::empty();
        let attribs : Vec<&str> = line.split(" ").collect();
        for value in attribs {
            if value.starts_with("info") {
                continue;
            } else {

                let value= value.split("=").collect::<Vec<&str>>();

                if value[0] == ("face") {
                    let name = value[1].replace("\"","");
                    info.name = name;
                } else if value[0] == ("size") {
                    info.size = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("bold") {
                    info.bold = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("italic") {
                    info.italic = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("charset") {
                    info.charset = value[1].replace("\"","");
                }else if value[0] == ("unicode") {
                    info.unicode = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("stretchH") {
                    info.stretchH = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("smooth") {
                    info.smooth = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("aa") {
                    info.aa = i32::from_str(value[1]).unwrap();
                }
            }
        };
        return info;
    }

    pub fn empty() -> InfoData {
        return InfoData {
            name: "".to_string(),
            size: 0,
            bold: 0,
            italic: 0,
            charset: "".to_string(),
            unicode: 0,
            stretchH: 0,
            smooth: 0,
            aa: 0
        };
    }
}

pub struct CommonData {
    pub line_height:i32,
    pub base:i32,
    pub scale_w:i32,
    pub scale_h:i32,
    pub pages:i32,
    pub packed:i32
}

impl CommonData {
    pub fn from_string(line:&str) -> CommonData {
        let mut info = CommonData::empty();
        let attribs : Vec<&str> = line.split(" ").collect();
        for value in attribs {
            if value.starts_with("common") {
                continue;
            }else {
                let value= value.split("=").collect::<Vec<&str>>();

                if value[0] == ("lineHeight") {
                    info.line_height = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("base") {
                    info.base = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("scaleW") {
                    info.scale_w = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("scaleH") {
                    info.scale_h = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("pages") {
                    info.pages = i32::from_str(value[1]).unwrap();
                }else if value[0] == ("packed") {
                    info.packed = i32::from_str(value[1]).unwrap();
                }
            }
        };
        return info;
    }
    pub fn empty() -> CommonData {
        return CommonData {
            line_height: 0,
            base: 0,
            scale_w: 0,
            scale_h: 0,
            pages: 0,
            packed: 0
        }
    }
}
 */
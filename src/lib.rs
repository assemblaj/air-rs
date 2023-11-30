use pest::error::Error;
use pest::Parser;
use pest::Token;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "air.pest"]
pub struct AIRParser;

#[derive(Default, Debug)]
pub struct Action {
    pub number: u64,
    pub elements: Vec<Element>,
    pub loop_start: usize,
    pub interpolates: Option<Vec<Interpolate>>,
}

#[derive(Debug, Default)]
pub struct Element {
    pub group: i64,
    pub image: i64,
    pub x: i64,
    pub y: i64,
    pub time: i64,
    pub flip: Option<Flip>,
    pub blend: Option<Blend>,
    pub clsn1: Option<Clsn>,
    pub clsn2: Option<Clsn>,
    pub rotation: Option<Rotation>,
    pub x_scale: Option<Scale>,
    pub y_scale: Option<Scale>,
}

impl Element {
    fn set_clsn1(&mut self, clsn1: Clsn) {
        self.clsn1 = Some(clsn1);
    }

    fn set_clsn2(&mut self, clsn2: Clsn) {
        self.clsn2 = Some(clsn2);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flip {
    Horizontal,
    Vertical,
    Both,
}

impl Flip {
    fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "HV" | "VH" => Flip::Both,
            "H" => Flip::Horizontal,
            "V" => Flip::Vertical,
            _ => panic!("Invalid flip parameter"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Blend {
    Add { src: u32, dst: u32 },
    Sub,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Interpolate {
    Offset(usize),
    Blend(usize),
    Scale(usize),
    Angle(usize),
}

impl Interpolate {
    fn new(interpolate_type: &str, element: usize) -> Self {
        let interpolate_type_lc = interpolate_type.to_lowercase();
        match interpolate_type_lc.as_str() {
            "offset" => Interpolate::Offset(element),
            "blend" => Interpolate::Blend(element),
            "scale" => Interpolate::Scale(element),
            "angle" => Interpolate::Angle(element),
            _ => panic!("Invalid interpolate"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ClsnBox(i64, i64, i64, i64);

impl ClsnBox {
    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }

    pub fn width(&self) -> i64 {
        self.2 - self.0
    }

    pub fn height(&self) -> i64 {
        self.3 - self.1
    }
}

#[derive(Debug, Clone)]
pub enum Clsn {
    Clsn1Default(Vec<ClsnBox>),
    Clsn2Default(Vec<ClsnBox>),
    Clsn1(Vec<ClsnBox>),
    Clsn2(Vec<ClsnBox>),
}

impl Clsn {
    fn new(clsn_type: &str, boxes: Vec<ClsnBox>) -> Self {
        let clsn_type_lc = clsn_type.to_lowercase();
        match clsn_type_lc.as_str() {
            "clsn2default" => Clsn::Clsn2Default(boxes),
            "clsn1default" => Clsn::Clsn1Default(boxes),
            "clsn1" => Clsn::Clsn1(boxes),
            "clsn2" => Clsn::Clsn2(boxes),
            _ => panic!("Invalid clsn."),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Scale(f64);

#[derive(PartialEq, Debug)]
pub struct Rotation(i64);

struct ElementBuilder {
    group: i64,
    image: i64,
    offset_x: i64,
    offset_y: i64,
    time: i64,
    flip: Option<Flip>,
    x_scale: Option<Scale>,
    y_scale: Option<Scale>,
    rotation: Option<Rotation>,
    blend: Option<Blend>,
}

impl ElementBuilder {
    fn new() -> ElementBuilder {
        ElementBuilder {
            group: 0,
            image: 0,
            offset_x: 0,
            offset_y: 0,
            time: 0,
            flip: None,
            rotation: None,
            x_scale: None,
            y_scale: None,
            blend: None,
        }
    }

    fn group(mut self, group: i64) -> Self {
        self.group = group;
        self
    }

    fn image(mut self, image: i64) -> Self {
        self.image = image;
        self
    }

    fn offset_x(mut self, offset_x: i64) -> Self {
        self.offset_x = offset_x;
        self
    }

    fn offset_y(mut self, offset_y: i64) -> Self {
        self.offset_y = offset_y;
        self
    }

    fn time(mut self, time: i64) -> Self {
        self.time = time;
        self
    }

    fn flip(mut self, flip: Flip) -> Self {
        self.flip = Some(flip);
        self
    }

    fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = Some(rotation);
        self
    }

    fn x_scale(mut self, scale: Scale) -> Self {
        self.x_scale = Some(scale);
        self
    }

    fn y_scale(mut self, scale: Scale) -> Self {
        self.y_scale = Some(scale);
        self
    }

    fn blend(mut self, blend: Blend) -> Self {
        self.blend = Some(blend);
        self
    }

    fn build(self) -> Element {
        Element {
            group: self.group,
            image: self.image,
            x: self.offset_x,
            y: self.offset_y,
            time: self.time,
            flip: self.flip,
            blend: self.blend,
            rotation: self.rotation,
            clsn1: None,
            clsn2: None,
            x_scale: self.x_scale,
            y_scale: self.y_scale,
        }
    }
}

struct ActionBuilder {
    number: u64,
    elements: Vec<Element>,
    loop_start: usize,
    current_clsn1: Option<Clsn>,
    current_clsn2: Option<Clsn>,
    current_element: Option<Element>,
    default_clsn1: Option<Clsn>,
    default_clsn2: Option<Clsn>,
    interpolates: Option<Vec<Interpolate>>,
}

impl ActionBuilder {
    fn new() -> ActionBuilder {
        ActionBuilder {
            number: 0,
            elements: Vec::new(),
            loop_start: 0,
            current_element: None,
            current_clsn1: None,
            current_clsn2: None,
            default_clsn1: None,
            default_clsn2: None,
            interpolates: None,
        }
    }

    fn number(mut self, number: u64) -> Self {
        self.number = number;
        self
    }

    fn element(mut self, mut element: Element) -> Self {
        if self.current_clsn1.is_some() {
            element.set_clsn1(self.current_clsn1.clone().unwrap());
            self.current_clsn1 = None;
        } else if self.default_clsn1.is_some() {
            element.set_clsn1(self.default_clsn1.clone().unwrap());
        }

        if self.current_clsn2.is_some() {
            element.set_clsn2(self.current_clsn2.clone().unwrap());
            self.current_clsn2 = None;
        } else if self.default_clsn2.is_some() {
            element.set_clsn2(self.default_clsn2.clone().unwrap());
        }

        self.elements.push(element);

        self
    }

    fn interpolate(mut self, interpolate: &str) -> Self {
        if self.interpolates.is_none() {
            self.interpolates = Some(Vec::new());
        }
        let mut interpolates = self.interpolates.unwrap();
        interpolates.push(Interpolate::new(interpolate, self.elements.len()));
        self.interpolates = Some(interpolates);
        self
    }

    fn loop_start(mut self) -> Self {
        self.loop_start = self.elements.len();
        self
    }

    fn clsn(mut self, clsn: Clsn) -> Self {
        match clsn {
            Clsn::Clsn1(_) => {
                self.current_clsn1 = Some(clsn);
            }
            Clsn::Clsn2(_) => {
                self.current_clsn2 = Some(clsn);
            }
            Clsn::Clsn1Default(_) => {
                self.default_clsn1 = Some(clsn);
            }
            Clsn::Clsn2Default(_) => {
                self.default_clsn2 = Some(clsn);
            }
        }
        self
    }

    fn build(self) -> Action {
        Action {
            number: self.number,
            elements: self.elements,
            loop_start: self.loop_start,
            interpolates: self.interpolates,
        }
    }
}

fn build_action(pairs: pest::iterators::Pairs<Rule>) -> Action {
    let mut action_builder = ActionBuilder::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::action_def => {
                let mut action_number_rule = pair.into_inner();
                let action_number_str = action_number_rule.next().unwrap().as_str();
                let action_number = str::parse(action_number_str).unwrap();
                action_builder = action_builder.number(action_number);
            }
            Rule::action_element => {
                let mut pair = pair.into_inner();
                let base_element_pair = pair.next().unwrap();
                let mut base_element_pairs = base_element_pair.into_inner();
                let group: i64 = str::parse(base_element_pairs.next().unwrap().as_str()).unwrap();
                let image: i64 = str::parse(base_element_pairs.next().unwrap().as_str()).unwrap();
                let offset_x: i64 =
                    str::parse(base_element_pairs.next().unwrap().as_str()).unwrap();
                let offset_y: i64 =
                    str::parse(base_element_pairs.next().unwrap().as_str()).unwrap();
                let time: i64 = str::parse(base_element_pairs.next().unwrap().as_str()).unwrap();
                let mut element_builder = ElementBuilder::new()
                    .group(group)
                    .image(image)
                    .offset_x(offset_x)
                    .offset_y(offset_y)
                    .time(time);

                if let Some(optional_element_pairs) = pair.next() {
                    let options_pairs = optional_element_pairs.into_inner();
                    for option in options_pairs {
                        match option.as_rule() {
                            Rule::flip => {
                                element_builder =
                                    element_builder.flip(Flip::from_str(option.as_str()));
                            }
                            Rule::blend => {
                                let blend_str = option.as_str();
                                let blend = match blend_str.to_uppercase().as_str() {
                                    "A" => Blend::Add { src: 256, dst: 256 },
                                    "A1" => Blend::Add { src: 256, dst: 128 },
                                    "S" => Blend::Sub,
                                    _ if blend_str.starts_with("AS") => {
                                        let mut blend_pairs = option.into_inner();
                                        let mut blend_elements =
                                            blend_pairs.next().unwrap().into_inner();
                                        let src: u32 =
                                            str::parse(blend_elements.next().unwrap().as_str())
                                                .unwrap();
                                        let dst: u32 =
                                            str::parse(blend_elements.next().unwrap().as_str())
                                                .unwrap();
                                        Blend::Add { src, dst }
                                    }
                                    _ => {
                                        panic!("Invalid blend parameter");
                                    }
                                };
                                element_builder = element_builder.blend(blend);
                            }
                            Rule::x_scale => {
                                let xs: f64 = str::parse(option.as_str()).unwrap();
                                element_builder = element_builder.x_scale(Scale(xs));
                            }
                            Rule::y_scale => {
                                let ys: f64 = str::parse(option.as_str()).unwrap();
                                element_builder = element_builder.y_scale(Scale(ys));
                            }
                            Rule::rotation => {
                                let rot: i64 = str::parse(option.as_str()).unwrap();
                                element_builder = element_builder.rotation(Rotation(rot));
                            }
                            _ => {
                                panic!("invalid action element option.")
                            }
                        }
                    }
                }

                let element = element_builder.build();
                action_builder = action_builder.element(element);
            }
            Rule::interpolation => {
                let mut pair = pair.into_inner();
                let interpolate = pair.next().unwrap();
                action_builder = action_builder.interpolate(interpolate.as_str());
            }
            Rule::clsn_group => {
                let mut pair = pair.into_inner();
                let clsn_def = pair.next().unwrap();
                let mut clsn_def_pair = clsn_def.into_inner();
                let clsn_type = clsn_def_pair.next().unwrap().as_str();
                let mut collision_boxes = Vec::new();
                let clsn_box = pair.next().unwrap();
                for pair in clsn_box.into_inner() {
                    match pair.as_rule() {
                        Rule::clsn_elements => {
                            let mut clsn_nums_pair = pair.into_inner();
                            let num1: i64 =
                                str::parse(clsn_nums_pair.next().unwrap().as_str()).unwrap();
                            let num2: i64 =
                                str::parse(clsn_nums_pair.next().unwrap().as_str()).unwrap();
                            let num3: i64 =
                                str::parse(clsn_nums_pair.next().unwrap().as_str()).unwrap();
                            let num4: i64 =
                                str::parse(clsn_nums_pair.next().unwrap().as_str()).unwrap();
                            collision_boxes.push(ClsnBox(num1, num2, num3, num4));
                        }
                        _ => {}
                    }
                }
                action_builder = action_builder.clsn(Clsn::new(clsn_type, collision_boxes));
            }
            Rule::loopstart => {
                action_builder = action_builder.loop_start();
            }
            _ => {
                panic!("Invalid elment: {:?} ", pair.as_rule());
            }
        }
    }

    action_builder.build()
}

pub fn parse(source: &str) -> Result<HashMap<u64, Action>, Error<Rule>> {
    let mut actions = HashMap::new();

    let files = AIRParser::parse(Rule::file, source)?;
    for file in files {
        let action_pairs = file.into_inner();
        for pair in action_pairs {
            match pair.as_rule() {
                Rule::action => {
                    let action = build_action(pair.into_inner());
                    actions.insert(action.number, action);
                }
                _ => {}
            }
        }
    }

    Ok(actions)
}

use eframe::{
    egui::{
        Align, CentralPanel, ComboBox, Context, FontSelection, Hyperlink, Layout, RichText,
        ScrollArea, Separator, TextEdit, TextStyle, TopBottomPanel, Ui,
    },
    epaint::{Color32, FontFamily, FontId},
    App, Frame,
};
use lib::{
    api::{self, Api, News},
    rd,
};

const PADDING: f32 = 10.0;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct UI {
    news_api: Api,
    filter: String,
}

impl UI {
    pub fn new(_: &eframe::CreationContext<'_>, api: Api) -> Self {
        Self {
            news_api: api,
            filter: "".to_owned(),
        }
    }

    pub fn get_s(&mut self) -> &mut String {
        &mut self.filter
    }

    pub fn api(&mut self) -> &mut Api {
        &mut self.news_api
    }
}

impl App for UI {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            design_header(self, ui);
            let mid = rd::read(&mut self.news_api);
            match mid {
                Ok(api) => match api.get_response() {
                    Some(res) => design_body(ui, res.get_articles(), &self.filter),
                    None => design_p(ui),
                },
                Err(_) => design_p(ui),
            }
            design_footer(ctx);
        });
    }
}

fn design_p(ui: &mut Ui) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.centered_and_justified(|ui| {
            ui.label("Unexpected Problem");
            ui.label("Retrying");
        });
    });
}

fn design_header(obj: &mut UI, ui: &mut Ui) {
    ui.add(
        TextEdit::singleline(obj.get_s())
            .desired_width(ui.available_width())
            .hint_text("search here")
            .frame(false),
    );

    let api = obj.api();
    let req = api.get_request_mut();

    let selected_ep = format!("{:?}", req.ep());
    let selected_cn = format!("{:?}", req.cn());

    let locs = api::Locs::get_countries();
    let fils = api::Fils::get_endpoints();

    ui.horizontal(|ui| {
        ComboBox::new("ep", "")
            .selected_text(selected_ep)
            .show_ui(ui, |ui| {
                for fil in fils {
                    ui.selectable_value(req.ep(), *fil, format!("{:?}", fil));
                }
            });

        ComboBox::new("cn", "")
            .selected_text(selected_cn)
            .show_ui(ui, |ui| {
                for loc in locs {
                    ui.selectable_value(req.cn(), *loc, format!("{:?}", loc));
                }
            });
    });
    ui.add_space(PADDING);
    ui.add(Separator::default().spacing(20.));
}

fn design_footer(ctx: &Context) {
    TopBottomPanel::bottom("end").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.label("On it");
        });
    });
}

fn design_body(ui: &mut Ui, items: &Vec<News>, fil: &str) {
    let items = filter(items, fil);

    ScrollArea::vertical().show(ui, |ui| {
        design_cards(ui, &items);
        ui.add_space(60.);
    });
}

fn design_cards(ui: &mut Ui, items: &Vec<&News>) {
    for (index, item) in items.iter().enumerate() {
        ui.push_id(format!("id = {}", index), |ui| {
            ui.push_id(format!("id = {}.0", index), |ui| {
                TextEdit::multiline(&mut item.get_title())
                    .text_color(Color32::WHITE)
                    .font(FontSelection::FontId(FontId::new(
                        20.,
                        FontFamily::Monospace,
                    )))
                    .desired_width(ui.available_width())
                    .desired_rows(1)
                    .show(ui);
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(format!("From: {}", item.get_source().get_name()));
                ui.label(" | ");
                ui.label(format!("Author: {}", item.get_author()));
            });

            ui.push_id(format!("id = {}.1", index), |ui| {
                TextEdit::multiline(&mut item.get_desc())
                    .code_editor()
                    .text_color(Color32::LIGHT_RED)
                    .font(FontSelection::FontId(FontId::new(
                        20.,
                        FontFamily::Monospace,
                    )))
                    .desired_width(ui.available_width())
                    .desired_rows(1)
                    .show(ui);
            });

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("read more ⤴")
                        .color(CYAN)
                        .size(16.)
                        .text_style(TextStyle::Small),
                    item.get_url(),
                ));
            });

            ui.add(Separator::default().spacing(10.));
        });
    }
}

#[allow(dead_code)]
fn filter<'a>(items: &'a Vec<News>, fil: &str) -> Vec<&'a News> {
    let mut con: Vec<(&News, i32)> = Vec::new();

    for item in items.iter() {
        let mut itg = 0;

        if item.get_title().contains(fil) {
            itg += 7;
        }
        if item.get_desc().contains(fil) {
            itg += 5;
        }

        if item.get_url().contains(fil) {
            itg += 3;
        }

        con.push((item, itg));
    }

    get(con)
}

fn get(mut vec: Vec<(&News, i32)>) -> Vec<&News> {
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    let mut ans: Vec<&News> = Vec::new();

    for index in 0..vec.len() {
        if vec[index].1 == 0 {
            break;
        }

        ans.push(vec.get(index).unwrap().0);
    }

    ans
}

use eframe::{
    egui::{
        Align, CentralPanel, CollapsingHeader, Context, Hyperlink, Label, Layout, RichText,
        ScrollArea, Separator, TextStyle, TopBottomPanel, Ui,
    },
    epaint::Color32,
    App, Frame,
};

use crate::api::{News, NewsApi};

const PADDING: f32 = 10.0;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct NewsUI {
    news_api: NewsApi,
    filter: String,
}

impl NewsUI {
    pub fn new(_: &eframe::CreationContext<'_>, api: NewsApi) -> NewsUI {
        NewsUI {
            news_api: api,
            filter: "".to_owned(),
        }
    }
}

impl App for NewsUI {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            design_header(ui, format!("{:?}", self.news_api.request.end_point));
            design_body(ui, self.news_api.mock(), &self.filter);
            design_footer(ctx);
        });
    }
}

fn design_header(ui: &mut Ui, label: String) {
    ui.vertical_centered(|ui| {
        ui.heading(label);
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
        ui.push_id(format!("{}", index), |ui| {
            ui.add_space(PADDING);
            ui.add(Separator::default().spacing(10.));

            //Adding the title of each news item
            ui.add(
                Label::new(
                    RichText::new(item.get_title())
                        .size(20.)
                        .text_style(TextStyle::Heading)
                        .strong()
                        .color(Color32::LIGHT_RED),
                )
                .wrap(true),
            );

            //Adding the body of each news item
            CollapsingHeader::new(
                RichText::new(item.get_desc())
                    .strong()
                    .size(18.)
                    .color(Color32::GOLD)
                    .text_style(TextStyle::Button),
            )
            .show(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.add(Hyperlink::from_label_and_url(
                        RichText::new("read more ⤴")
                            .color(CYAN)
                            .text_style(TextStyle::Monospace),
                        item.get_url(),
                    ));
                });
            });
        });
    }
}

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

        ans.push(vec.get(0).unwrap().0);
    }

    ans
}

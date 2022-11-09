use eframe::{
    egui::{
        CentralPanel, CollapsingHeader, Context, Hyperlink, Label, RichText, ScrollArea, Separator,
        TopBottomPanel, Ui, Area,
    },
    epaint::Color32,
    App, Frame,
};

use crate::api::{Fils, News, NewsApi};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct NewsUI {
    news_api: NewsApi,
}

impl NewsUI {
    pub fn new(_: &eframe::CreationContext<'_>, api: NewsApi) -> NewsUI {
        NewsUI { news_api: api }
    }
}

impl App for NewsUI {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(PADDING);
            design_header(ui, self.news_api.request.end_point);
            ScrollArea::vertical().show(ui, |ui| {
                let arr = self.news_api.fetch();
                for news in arr.iter() {
                    design_newsitem(ui, news);
                    let sep = Separator::default().spacing(20.);
                    ui.add(sep);
                }
                ui.add_space(PADDING);
            });
            design_footer(ui, ctx);
        });
    }
}

fn design_header(ui: &mut Ui, end_point: Fils) {
    let header = Label::new(RichText::new(&format!("{:?}", end_point))
    .size(20.)
    .heading()
    .strong()
    .color(Color32::RED));

    ui.add(header);
    ui.add_space(PADDING);
}

fn design_footer(ui: &mut Ui, ctx: &Context) {
    ui.add_space(PADDING);
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add(Label::new("newsapi.org"));
            ui.add_space(10.);
        });
    });
}

pub fn design_newsitem(ui: &mut Ui, news: &News) {
    CollapsingHeader::new(
        RichText::new(&news.title)
            .size(20.)
            .heading()
            .strong()
            .color(WHITE),
    )
    .show(ui, |ui| {
        ui.add_space(PADDING);
        let sep = Separator::default().spacing(18.);
        ui.add(sep);

        ui.label(
            RichText::new(&news.description)
                .size(20.)
                .color(Color32::YELLOW),
        );

        ui.add(Hyperlink::from_label_and_url(
            RichText::new("read more ⤴").color(CYAN).size(16.),
            &news.url,
        ));
    });
}

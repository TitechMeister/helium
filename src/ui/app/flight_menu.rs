use eframe::egui;

pub struct FlightMenu {
    pub cnt: u8,
    pub menu: String,
    pub customize: bool,
}

impl FlightMenu {
    pub fn new() -> Self {
        FlightMenu {
            cnt: 1,
            menu: "転がし".to_string(),
            customize: false,
        }
    }
}

impl super::AppUI for FlightMenu {
    fn update(&mut self, __data: &crate::parse::Parser, ctx: &eframe::egui::Context) {
        egui::Window::new(format!("Flight Menu"))
            .show(ctx, |ui| {
                ui.horizontal(|ui|{
                    ui.heading(format!("{}回目",self.cnt));
                    if self.customize{
                        ui.text_edit_singleline(&mut self.menu);
                    }else{
                        ui.heading(&self.menu);
                    }
                });
                ui.add_space(10.0);
                ui.label(format!("{:?}", chrono::Local::now()));
                ui.add_space(10.0);
                ui.horizontal( |ui| {
                    for l in [
                        "転がし",
                        "滑走",
                        "ジャンプ",
                        "短距離",
                        "中距離",
                        "長距離",
                        "飛び切り",
                    ] {
                        if ui.selectable_value(&mut self.menu, l.to_string(), l).clicked(){
                            self.customize=false;
                        }
                    }
                    ui.checkbox(&mut self.customize, "custom");
                });
                ui.add_space(0.0);
                ui.horizontal(|ui|{
                    let mut p=ui.max_rect().max;
                    p.x/=2.0;
                    if ui.put(egui::Rect::from_min_max(egui::Pos2 { x: ui.min_rect().min.x, y: ui.max_rect().max.y }, p), egui::Button::new("Prev")).clicked() && self.cnt>1{
                        self.cnt-=1
                    }
                    if ui.put(egui::Rect::from_min_max( p,ui.max_rect().max), egui::Button::new("Next")).clicked(){
                        self.cnt+=1
                    };
                });
            });
    }
}

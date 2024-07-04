  fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    native_options.initial_window_size = Some(egui::Vec2 {x:400.0, y:200.0});
  
    let _ = eframe::run_native("My egui App", native_options,
      Box::new(|cc| Box::new(MyEguiApp::new(cc))));
  }

  struct MyEguiApp {
    pub value:usize,
  }
  
  impl Default for MyEguiApp {
    //インスタンス作成時にvalue初期化するためDefault
    fn default() -> MyEguiApp {
      MyEguiApp{
        value:0,
      }
    }
  }
  
  impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
      Self::default()
    }
  }

  impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello World!");
        
        ui.spacing();
  
        let msg = format!("click {} times.", self.value);
        let label_txt = egui::RichText::new(msg)
          .size(32.0);
        let label = egui::Label::new(label_txt);
        ui.add(label);
  
        ui.separator();
  
        let btn_txt = egui::RichText::new("Click!").font(egui::FontId::proportional(24.0));
        let btn = egui::Button::new(btn_txt);
        let resp = ui.add_sized(egui::Vec2 {x:150.0, y:40.0}, btn);
        if resp.clicked() {
          self.value += 1;
        }
      });
    }
  }
use ggez::graphics;

pub struct Button {
    label: graphics::Text,
    rect: graphics::Rect,
    hover: bool,
    press: bool,
}

impl Button {
    pub fn new(label: &str, rect: graphics::Rect) -> Button {
        let label = graphics::Text::new(label.to_owned()); 
        
        Button {
            label,
            rect,
            hover: false,
            press: false,
        }
    }

    pub fn draw(self: &Button, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut mesh_builder = graphics::MeshBuilder::new();

        let (fill_color, border_color) = match self.hover {
            true => 
                match self.press {
                    true => (graphics::Color {
                                    r: 0.4_f32,
                                    g: 0.4_f32,
                                    b: 0.4_f32,
                                    a: 1_f32,
                                },
                                graphics::Color::YELLOW,
                            ),
                    false =>(graphics::Color {
                                    r: 0.3_f32,
                                    g: 0.3_f32,
                                    b: 0.3_f32,
                                    a: 1_f32,
                                },
                                graphics::Color::WHITE,
                            )
                }
            ,

            false => (graphics::Color {
                    r: 0.1_f32,
                    g: 0.1_f32,
                    b: 0.1_f32,
                    a: 1_f32,
                },
                graphics::Color::WHITE,
                )
        };

        mesh_builder.rectangle(graphics::DrawMode::Fill(graphics::FillOptions::default()), self.rect, fill_color)?;
        mesh_builder.rectangle(graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(2_f32)), self.rect, border_color)?;
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        let label_position = glam::vec2(self.rect.x + self.rect.w / 2_f32 - self.label.width(ctx) / 2_f32, self.rect.y + self.rect.h / 2_f32 - self.label.height(ctx) / 2_f32);
        let label_draw_param = graphics::DrawParam::default().dest(label_position);
        graphics::draw(ctx, &self.label, label_draw_param)?;

        ggez::GameResult::Ok(())
    }

    pub fn update(self: &mut Button, ctx: &ggez::Context) -> bool {
        let mouse_position = ggez::input::mouse::position(ctx);
        self.hover = self.rect.contains(mouse_position);

        let was_pressed = self.press;
        let mouse_down = ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left);
        self.press =  self.hover && mouse_down;
        if was_pressed && !self.press && self.hover {
            return true;
        }

        return false;
    }
}
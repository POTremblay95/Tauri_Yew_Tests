use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub enum PlotMsg {
    Redraw,
    Nothing,
}

/*
#[derive(Properties, PartialEq)]
pub struct PlotProps {}
*/
pub struct Plot {
    canvas: NodeRef,
    draw_x2: bool,
}

impl Plot {
    fn get_label(&self) -> Option<String> {
        if self.draw_x2 {
            Some(String::from("y = x^2"))
        } else {
            Some(String::from("y = sin(x)"))
        }
    }
}

impl Component for Plot {
    type Message = PlotMsg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
          <div style="width: 100%; height: 100%;">
            <canvas style="width: 100%; height: 100%;" onclick={ctx.link().callback(|_| PlotMsg::Redraw)} ref = {self.canvas.clone()}/>
          </div>
        )
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PlotMsg::Nothing => true,
            _ => {
                let element: HtmlCanvasElement = self.canvas.cast().unwrap();

                let rect = element.get_bounding_client_rect();
                element.set_height(rect.height() as u32);
                element.set_width(rect.width() as u32);

                let backend = CanvasBackend::with_canvas_object(element).unwrap();

                let drawing_area = backend.into_drawing_area();
                drawing_area.fill(&RGBColor(200, 200, 200)).unwrap(); // Gray background

                let plot_label = self.get_label().expect("There was an error.");

                let mut chart = ChartBuilder::on(&drawing_area)
                    .caption(&plot_label, ("sans-serif", 14).into_font())
                    .margin(10)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(-1f32..1f32, -1f32..1f32) //x axis range and y axis range
                    .unwrap();

                chart.configure_mesh().draw().unwrap();

                if self.draw_x2 {
                    chart
                        .draw_series(LineSeries::new(
                            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                            &RED,
                        ))
                        .unwrap()
                        .label(&plot_label)
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                } else {
                    chart
                        .draw_series(LineSeries::new(
                            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x.sin())),
                            &RED,
                        ))
                        .unwrap()
                        .label(&plot_label)
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                }
                // Change the member for next redraw
                self.draw_x2 = !self.draw_x2;

                false
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(PlotMsg::Redraw);
        Plot {
            canvas: NodeRef::default(),
            draw_x2: true,
        }
    }
}

fn main() {
    yew::Renderer::<Plot>::new().render();
}

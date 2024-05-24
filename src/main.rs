use vizia::prelude::*;

#[derive(Lens, Clone, Data)]
pub struct AppData {
    current: Task,
    tasks: Vec<Task>,
}

pub enum AppEvent {
    Send,
    SetHeading(String),
    SetDesc(String),
    Remove(usize),
}

#[derive(Lens, Clone, Data)]
pub struct Task {
    heading: String,
    desc: String,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Send => {
                self.tasks.push(Task {
                    heading: self.current.heading.clone(),
                    desc: self.current.desc.clone(),
                });
            }
            AppEvent::SetHeading(s) => {
                self.current.heading = s.to_string();
            }
            AppEvent::SetDesc(s) => {
                self.current.desc = s.to_string();
            }
            AppEvent::Remove(i) => {
                self.tasks.remove(*i);
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData {
            current: Task {
                heading: "hi".to_string(),
                desc: "first task".to_string(),
            },
            tasks: vec![Task {
                heading: "hi".to_string(),
                desc: "first task".to_string(),
            }],
        }
        .build(cx);

        HStack::new(cx, |cx| {
            List::new(cx, AppData::tasks, |cx, index, item| {
                VStack::new(cx, |cx| {
                    Label::new(cx, item.map(|task| format!("{}, ", task.heading)))
                        .class("heading")
                        .class("raised");
                    Label::new(cx, item.map(|task| format!("{}, ", task.desc))).class("raised");
                })
                .height(Pixels(50.0))
                .child_space(Stretch(1.0));

                Button::new(cx, |cx| Label::new(cx, "Remove"))
                    .on_press(move |cx| cx.emit(AppEvent::Remove(index.clone())));
            })
            .width(Pixels(1000.0))
            .child_space(Stretch(1.0))
            .col_between(Pixels(800.0))
            .live(Live::Assertive);

            VStack::new(cx, |cx| {
                Textbox::new(cx, AppData::current.then(Task::heading)).on_edit(move |cx, text| {
                    cx.emit(AppEvent::SetHeading(text.clone()));
                });
                Textbox::new_multiline(cx, AppData::current.then(Task::desc), true).on_edit(
                    move |cx, text| {
                        cx.emit(AppEvent::SetDesc(text.clone()));
                    },
                );
            });

            Button::new(cx, |cx| Label::new(cx, "+"))
                .on_press(|cx| cx.emit(AppEvent::Send))
                .class("blue");
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(50.0));
    })
    .title("Tasks")
    .inner_size((800, 450))
    .run()
    .expect("problem");
}

const STYLE: &str = r#"
    :root {
        
        font-family: "Outfit";
        font-weight: 500;
        background-color: #131415;
        
        color: white;
    }

    .blue {
        background-color: #586aea;
    }

    .heading {
        font-weight: 800;
        font-size: xx-large;
    }

    .raised {
        background-color: #1a1c1e;
    }
    button {
        background-color: #1a1c1e;
    }
    textbox {
        background-color: #1a1c1e;
        color: white;
    }

"#;

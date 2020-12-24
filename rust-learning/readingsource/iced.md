# iced

## about

### q1:

 iced/examples/events/src/main.rs

```
pub fn main() {
    Events::run(Settings::default())
}
```
Where is 'Events::run'.


- iced/src/application.rs:
    fn run(_settings: Settings)
    where
        Self: 'static,
    {
        #[cfg(not(target_arch = "wasm32"))]
        <Instance<Self> as iced_winit::Application>::run(
            _settings.into(),
            iced_wgpu::Settings {
                default_font: _settings.default_font,
                antialiasing: if _settings.antialiasing {
                    Some(iced_wgpu::settings::Antialiasing::MSAAx4)
                } else {
                    None
                },
            },
        );

        #[cfg(target_arch = "wasm32")]
        <Instance<Self> as iced_web::Application>::run();
    }

- iced/src/sandbox.rs
    fn run(settings: Settings)
    where
        Self: 'static + Sized,
    {
        <Self as Application>::run(settings)
    }	

- iced/web/src/lib.rs

   fn run()
    where
        Self: 'static + Sized,
    {
        use futures::stream::StreamExt;

        let (app, command) = Self::new();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let mut title = app.title();
        document.set_title(&title);

        let (sender, receiver) =
            iced_futures::futures::channel::mpsc::unbounded();

        let mut runtime = iced_futures::Runtime::new(
            Self::Executor::new().expect("Create executor"),
            sender.clone(),
        );
        runtime.spawn(command);

        let application = Rc::new(RefCell::new(app));

        let instance = Instance {
            application: application.clone(),
            bus: Bus::new(sender),
        };

        let vdom = dodrio::Vdom::new(&body, instance);

        let event_loop = receiver.for_each(move |message| {
            let command = application.borrow_mut().update(message);
            let subscription = application.borrow().subscription();
            let new_title = application.borrow().title();

            runtime.spawn(command);
            runtime.track(subscription);

            if title != new_title {
                document.set_title(&new_title);

                title = new_title;
            }

            vdom.weak().schedule_render();

            futures::future::ready(())
        });

        wasm_bindgen_futures::spawn_local(event_loop);
    }	

- iced/winit/src/application.rs

    fn run(
        settings: Settings,
        backend_settings: <Self::Backend as window::Backend>::Settings,
    ) where
        Self: 'static,
    {
	
		...
	
	}

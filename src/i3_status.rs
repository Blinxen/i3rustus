use crate::{
    config::Config,
    widget_executor::{UpdateWidgetValue, WidgetExecutor, WidgetValue},
    widgets::Widget,
};
use actix::{Actor, Addr};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::{thread, time};

pub struct I3Status {
    config: Config,
    widget_executors: HashMap<String, Addr<WidgetExecutor>>,
}

impl I3Status {
    pub fn new(config: Config, widgets: Vec<Box<dyn Widget>>) -> Self {
        let mut executors = HashMap::new();

        for widget in widgets {
            // TODO: I don't like the to_string here
            executors.insert(
                widget.name().to_string(),
                WidgetExecutor::new(widget).start(),
            );
        }

        I3Status {
            config,
            widget_executors: executors,
        }
    }

    async fn widget_values(&self) -> Value {
        let mut values = json!([]);
        // Make sure widgets are printed in the correct order
        for widget_name in self.config.widget_order() {
            match self
                .widget_executors
                .get(&widget_name)
                .expect("ERROR: Unknown widget name")
                .send(WidgetValue {})
                .await
            {
                Ok(Ok(conf)) => values
                    .as_array_mut()
                    .expect("ERROR: Could not get a mutable Vec from serde JSON")
                    .push(conf),
                Ok(Err(error)) => {
                    log::warn!("Invalid value for {}: \n\t{}", widget_name, error);
                    continue;
                }
                _ => {
                    log::error!(
                        "Unexpected error when trying to get the value of {}!",
                        widget_name
                    );
                    continue;
                }
            };
        }

        values
    }

    fn update_widgets(&self) {
        // Send update message to all executors
        // This will start a "update" job
        for executor in self.widget_executors.values() {
            executor.do_send(UpdateWidgetValue {});
        }
    }

    pub async fn init(&mut self) {
        // Make sure all widgets contain a valid value before starting the actual loop
        self.update_widgets();
        // This is the output that is read by i3
        println!("{{\"version\":1}}");
        // Begin endless array
        println!("[");
        // Arrays have to be separated by comma in output
        println!("[]");
        loop {
            // Print all values, these values will be seen in i3bar
            println!(",{}", self.widget_values().await);
            // Wait 1 secs before printing update
            thread::sleep(time::Duration::from_secs(1));
            self.update_widgets();
        }
    }
}
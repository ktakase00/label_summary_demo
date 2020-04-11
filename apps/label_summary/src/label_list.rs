use std::collections::HashMap;
use crate::issue::Issue;

#[derive(Debug)]
pub struct LabelList<'a> {
    issue_list: &'a Vec<Issue>,
    summary: HashMap<String, usize>,
}

impl<'a> LabelList<'a> {
    pub fn new(issue_list: &'a Vec<Issue>) -> LabelList<'a> {
        let summary = issue_list.iter().fold(HashMap::new(), |acc, issue| {
            issue.labels().iter().fold(acc, |mut label_map, label| {
                let count = label_map.entry(label.to_string()).or_insert(0);
                *count += 1;
                label_map
            })
        });
        LabelList {
            issue_list,
            summary,
        }
    }

    pub fn format(&self) -> String {
        self.summary.iter().fold(String::new(), |mut content, item| {
            content.push_str(format!("{}: {}\n", item.0, item.1).as_str());
            content
        })
    }
}

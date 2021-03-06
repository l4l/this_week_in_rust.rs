use std::fmt;

use crate::select;
use crate::select::predicate::Name;

use crate::html::escape;

#[derive(Debug)]
pub struct Link {
    link: String,
    text: String,
}

impl Link {
    pub fn from_node<'a>(node: select::node::Node<'a>) -> Link {
        let text = node.text().replace(". [discuss]", "");
        let link = node.find(Name("a")).next().unwrap().attr("href").unwrap();
        Link {
            link: escape(link.to_string()),
            text: escape(text),
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{text}\n{link}\n", link = self.link, text = self.text)
    }
}

#[derive(Debug)]
pub struct LinksList(Vec<Link>);

impl fmt::Display for LinksList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stringified = self
            .0
            .iter()
            .map(|link| format!("{}", link))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", stringified)
    }
}

#[derive(Debug)]
pub struct News(LinksList);

impl News {
    pub fn new(list: Vec<Link>) -> Self {
        News(LinksList(list))
    }
}

impl fmt::Display for News {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<b>News</b>\n\n{}", self.0)
    }
}

#[derive(Debug)]
pub struct CrateOfWeek {
    pub name: String,
    pub text: String,
    pub link: String,
}

impl fmt::Display for CrateOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<b>Crate of the week:</b> <a href=\"{link}\">{name}</a>\n\n{text}\n",
            link = self.link,
            name = self.name,
            text = self.text
        )
    }
}

#[derive(Debug)]
pub struct Updates(LinksList);

impl Updates {
    pub fn new(list: Vec<Link>) -> Self {
        Updates(LinksList(list))
    }
}

impl fmt::Display for Updates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<b>Updates from core</b>\n\n{}", self.0)
    }
}

#[derive(Debug)]
pub struct Article {
    pub(super) id: i32,
    pub(super) date: String,
    pub(super) link: String,
    pub(super) news: News,
    pub(super) crate_of_week: CrateOfWeek,
    pub(super) updates: Updates,
}

impl Article {
    pub fn head(&self) -> String {
        format!(
            "<b>This week in Rust #{id}</b> — {date}\n\n{link}",
            id = self.id,
            link = self.link,
            date = self.date.to_lowercase()
        )
    }

    pub fn news(&self) -> String {
        format!("{}", self.news)
    }

    pub fn crate_of_week(&self) -> String {
        format!("{}", self.crate_of_week)
    }

    pub fn core_updates(&self) -> String {
        format!("{}", self.updates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_fmt() {
        let link = Link {
            link: "FOO".to_string(),
            text: "BAR".to_string(),
        };

        assert_eq!(format!("{}", link), "BAR\nFOO\n".to_string());
    }

    #[test]
    fn links_fmt() {
        let links = LinksList(vec![
            Link {
                link: "linkA".to_string(),
                text: "textA".to_string(),
            },
            Link {
                link: "linkB".to_string(),
                text: "textB".to_string(),
            },
            Link {
                link: "linkC".to_string(),
                text: "textC".to_string(),
            },
        ]);

        let expected = "textA\nlinkA\n\ntextB\nlinkB\n\ntextC\nlinkC\n".to_string();

        assert_eq!(format!("{}", links), expected);
    }
}

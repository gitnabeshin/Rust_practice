// Trait definition
pub trait Summary {
    fn summarize(&self) -> String;
    // this default function can overwrite.
    fn summarize_def(&self) -> String {
        String::from("(This is default content....)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub locarion: String,
    pub author: String,
    pub content: String,
}

//Implementing a Trait on a Type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.locarion)
    }
    fn summarize_def(&self) -> String {
        format!("[OVER WRITE]:{}", self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//Implementing a Trait on a Type
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait as param of function
pub fn notify(item: &impl Summary) {
    println!("NOTIFY: {}", item.summarize());
}

// Clearer Trait Bounds with where Clauses
pub fn notify2<T: Summary>(item: &T) {
    println!("NOTIFY trait bound: {}", item.summarize_def());
}

// use std::fmt::Display;
// Clearer Trait Bounds with where Clauses
// pub fn notify3<T: Summary + Display>(item: &T) {
//     println!("DISPLAY: {}", item);
// }

pub fn get_summarizable() -> impl Summary {
    Tweet {
        username: String::from("ret_user"),
        content: String::from("ret content"),
        reply: false,
        retweet: false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let tweet = Tweet {
            username: String::from("user_1"),
            content: String::from("I'll find some today's hot topic."),
            reply: false,
            retweet: false,
        };

        println!("1: Tweet: {}", tweet.summarize());
        println!("1: >>> {}", tweet.summarize_def());
        notify(&tweet);
        notify2(&tweet);
        // notify3(&tweet);
        assert_eq!(tweet.reply, false);
    }

    #[test]
    fn it_works2() {
        let news = NewsArticle {
            headline: String::from("This is Headline."),
            locarion: String::from("Yokohama Japan"),
            author: String::from("nabeshin"),
            content: String::from("today's weather report."),
        };

        println!("2: News: {}", news.summarize());
        println!("2: >>> {}", news.summarize_def());
        notify(&news);
        notify2(&news);
        // notify3(&news);
        assert_eq!(news.author, "nabeshin");
    }

    #[test]
    fn it_works3() {
        let ret = get_summarizable();

        println!("3: News: {}", ret.summarize());
        println!("3: >>> {}", ret.summarize_def());
        notify(&ret);
        notify2(&ret);
    }
}

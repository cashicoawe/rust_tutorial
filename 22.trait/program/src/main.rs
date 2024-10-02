pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("read more")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// impl Tweet {
//     fn summarize(&self) -> String{
//         format!("{}: {}", self.username, self.content)
//     }
// }

pub fn notify(item: &impl Summary) {
    println!("notify item.summarize(): {:?}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("notify2 item.summarize(): {:?}", item.summarize());
}

pub fn notify3<T: Summary, U: Summary>(item1: &T, item2: &U){
    println!("\nnotify3\nitem1: {:?}\nitem2: {:?}", item1.summarize(), item2.summarize());
}

pub trait Display{
    fn show(&self) -> String {
        String::from("no contents to show")
    }
}

pub struct Test {
    pub text: String
}

impl Summary for Test {}
impl Display for Test {}

pub fn notify4<T: Summary + Display>(item: &T){
    println!("\nnotify4:\nsummarize: {:?}\nshow: {:?}", item.summarize(), item.show());
}

pub fn notify5<T, U>(item1: &T, item2: &U) where T:Summary + Display, U: Summary{
    println!("\nnotify5");
    println!("item1: summarize: {:?}, show: {:?}", item1.summarize(), item1.show());
    println!("item2: summarize: {:?}", item2.summarize());
}

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
// https://doc.rust-jp.rs/book-ja/ch17-02-trait-objects.html#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E3%81%A7%E7%95%B0%E3%81%AA%E3%82%8B%E5%9E%8B%E3%81%AE%E5%80%A4%E3%82%92%E8%A8%B1%E5%AE%B9%E3%81%99%E3%82%8B

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("maybe..."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content.............."),
    };

    println!("news.summarize(): {:?}", news.summarize());

    notify(&tweet);
    notify(&news);
    notify2(&tweet);
    notify2(&news);
    notify3(&tweet, &news);

    let t = Test {
        text: String::from("hello"),
    };
    notify4(&t);
    notify5(&t, &tweet);
}

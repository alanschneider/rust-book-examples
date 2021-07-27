pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait SummaryWithDefaultImplementation {
    fn summarize_author(&self) -> String;

    fn default_summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}

impl SummaryWithDefaultImplementation for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
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

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub trait Display {
    fn display(&self) {
        // default to do nothing
    }
}

impl Display for Tweet {}

pub trait Debug {}

// Using a where clause for a less cluttered definition
//
fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

pub fn display_summary(_item: &(impl Summary + Display)) {
    println!("implements Display and Summary");
}

// Returning types that implement Traits
//
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("MrEd"),
        content: String::from("a horse is a horse, of course, of course"),
        reply: false,
        retweet: false,
    };

    println!("tweet.summarize(): {}", tweet.summarize());
    display_summary(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh Slim"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("article.summarize(): {}", article.summarize());
    println!(
        "article.default_summarize(): {}",
        article.default_summarize()
    );
    // display_summary(&article); // fails, since NewsArticle doesn't implement Display

    notify(&tweet);
    notify(&article);

    let s = returns_summarizable(true).summarize();
    println!("{}", s);
}

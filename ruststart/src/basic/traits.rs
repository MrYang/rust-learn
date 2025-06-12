trait Summary {
    fn summarize(&self) -> String;
    fn summary_default(&self) -> String {
        format!("summary default trait")
    }
}

trait Display {
    fn display(&self) -> String {
        format!("display trait")
    }
}

#[derive(Debug)]
struct Article {
    headline: String,
    location: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("article :{:?}", self)
    }
}

impl Display for Article {}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("tweet :{:?}", self)
    }
}

fn tr(args: &impl Summary) {
    println!("summary:{}", args.summarize())
}

fn tr2<T: Summary>(args: &T) {
    println!("summary:{}", args.summary_default())
}

fn tr3<T: Summary + Display>(args: &T) {
    println!("multi trait, summary:{},display:{}", args.summary_default(), args.display())
}

fn return_summary() -> impl Summary {
    Tweet {
        username: String::from("username"),
        content: String::from("content"),
    }
}

pub fn traits() {
    let mut a = Article {
        headline: String::from("headline"),
        location: String::from("location"),
    };

    let mut t = Tweet {
        username: String::from("username"),
        content: String::from("content"),
    };

    println!("a.summarize:{}, a.summary:{}, a.display:{}", a.summarize(), a.summary_default(), a.display());
    tr(&mut a);
    tr(&mut t);

    tr2(&mut a);

    tr3(&mut a);

    let s = return_summary();
    println!("s.summarize:{}", s.summary_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traits() {
        traits();
    }
}
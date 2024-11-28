struct Generator {
    key: &'static str,
}

impl Generator {
    fn new(key: &'static str) -> Self {
        Self { key }
    }

    fn generate(&self) -> String {
        format!(
            "{}.{}",
            self.key,
            std::str::from_utf8(&tuid::once().as_hex().unwrap()).unwrap()
        )
    }

    // fn parse(&self, id: &str) -> tuid::Tuid {
    //     let idx = self.key.len();
    //     let ns = &id[..idx];
    //     let tid = &id[idx + 1..];
    //     assert_eq!(ns, self.key, "key doesn't match");
    //     tuid::Tuid::from_hex(&tid.as_bytes()[..16]).expect("invalid character")
    // }
}

// cargo run -p tuid-examples --example namespace
fn main() {
    let articles = Generator::new("ar");
    let comments = Generator::new("cm");

    let article_id = articles.generate();
    let comment_id = comments.generate();

    println!("article id: {}", article_id);
    println!("comment id: {}", comment_id);

    // let article_id = articles.parse(&article_id).as_hex();
    // let comment_id = comments.parse(&comment_id).as_hex();

    // println!("article id: {}", article_id);
    // println!("comment id: {}", comment_id);
}

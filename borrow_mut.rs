#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I borrowed {} {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
}

fn main() {
    let b = Book {
        author: "邹澍",
        title: "有趣的统计",
        year: 2015,
    };
    borrow_book(&b);
    //new_edition(&mut b);
    let mut mut_b = b;
    new_edition(&mut mut_b);
    borrow_book(&mut_b);
}
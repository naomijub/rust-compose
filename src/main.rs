fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add_and_multiply = compose(|x| x * 3f32, |x| x + 3f32);
    let divide_and_subtract = compose(|x| x / 3f32, |x| x - 3f32);

    let composed = compose(add_and_multiply, divide_and_subtract);
    println!("Result is {}", composed(20f32));
}
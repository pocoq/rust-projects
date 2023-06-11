#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

pub fn About(cx: Scope) -> Element {
    cx.render(rsx! (p{
        b{"Dioxus Lab"}
        " An Open Source project dedicated to making Rust UI wonderful."
    }))
}

#[derive(Props)]
struct ExplicitOptionProps<'a> {
    title: &'a str,
    #[props(!optional)]
    subtitle: Option<&'a str>,
}

fn ExplicitOption<'a>(cx: Scope<'a, ExplicitOptionProps>) -> Element<'a> {
    cx.render(rsx!(h1 {
        "{cx.props.title}: ",
        cx.props.subtitle.unwrap_or("No subtitle provided"),
    }))
}

#[derive(Props)]
struct OptionalProps<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
}

fn Title<'a>(cx: Scope<'a, OptionalProps>) -> Element<'a> {
    cx.render(rsx! (h1{
        "{cx.props.title}: ",
        cx.props.subtitle.unwrap_or("No subtitle provided"),
    }))
}

#[derive(Props)]
struct TitleCardProps<'a> {
    title: &'a str,
}

fn TitleCard<'a>(cx: Scope<'a, TitleCardProps<'a>>) -> Element {
    cx.render(rsx! {
        h1{"{cx.props.title}"}
    })
}

#[derive(PartialEq, Props)]
struct LikesProps {
    score: i32,
}

fn Likes(cx: Scope<LikesProps>) -> Element {
    cx.render(rsx! {
        div{
            "This post has ", b{"{cx.props.score}"}, " likes"
        }
    })
}

fn App(cx: Scope) -> Element {
    let coordinate = (42, 0);
    let country = "es";
    let text = "dioxus";
    let contents = "live <b> dangerously </b>";
    let hello = "Hello Dioxus !";
    cx.render(rsx! {
        div {
            // hidden: "false",
            "hello world"
            // dangerous_inner_html: "{contents}"
        }
        /*********************************** */
        if true{
            rsx!(div {"true"})
        }
        span{
            text.to_uppercase(),
            (0..10).map(|i| rsx!{ i.to_string() })
        }
        div {
            for i in 0..3 {
                div {"{i}"}
            }
        }
        div {
            (0..3).map(|i| rsx!{ div { "{i}" } })
        }
        div{
            class: "country-{country}",
            "position":"{coordinate:?}",
            div{
                "{country.to_uppercase()}"
            },
            div{
                "{7*6}"
            }
            div{
                {{}}
            }
            a{
                href: "/local",
                class: "primary",
                color: "red",
                "testing",
            }
            "Hello, world"
        }
        /*********************************** */
        About {},
        About {},
        /*********************************** */
        Likes{score: 42},
        TitleCard{title: hello}
        Title{title: "Some title", subtitle: "Some subtitle"},
        Title{title: "Some title without subtile"},

		ExplicitOption{title: "Some title", subtitle: Some("Some subtitle")},
        ExplicitOption{title: "Some title without subtile", subtitle: None},
    })
}

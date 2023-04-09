use leptos::*;
use leptos_router::*;

#[component]
#[allow(non_snake_case)]
pub fn About(_cx: Scope) -> Element {
    view! {
        _cx,
        <div>
            <div class="data">
                <div class="intro">
                    <p>"hi there again"</p>
                    <br/>
                    <p>"i&apos;m trying to be an any-stack developer, which means i&apos;d love to explore new stacks, languages, ... in order to build more awesome, performant project."</p>
                    <p>"i&apos;m using Arch linux btw, with Awesomewm; coding JS, Rust in Neovim; typing on an 65% keyboard and working remotely in my tiny cozy room."</p>
                    <p>" i love watching tv series, youtube videos, twitch streams, writing things, sometimes write songs, running, swimming, badminton, chess ..."</p>
                    <br />
                    <p>"so, click if you"</p>
                    <ul>
                        <li><a href="https://www.linkedin.com/in/huy-le-vu-minh/" target="_blank">"think there is a job that suitable for me"</a></li>
                        <li><a href="https://github.com/LeVuMinhHuy" target="_blank">"wanna check out some of my projects"</a></li>
                        <li><a href="https://www.facebook.com/moreromem" target="_blank">"comfortable to drop some messages"</a></li>
                        <li><a href="/">"or just want to read"</a></li>

                    </ul>

                    <br/>
                    <p>"i&apos;m also proud to say that this site is built with zero line of js. it's all about rust and wasm, based on leptos"</p>
                </div>

                <nav class="nav">
                    <div class="nav-text">
                       <A exact=true href="/blog"><p>"# blog"</p></A>
                       <A exact=true href="/"><p>"# home"</p></A>
                    </div>
                </nav>
            </div>
        </div>
    }
}

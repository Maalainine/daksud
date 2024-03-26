#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

fn main() {
    // Urls are relative to your Cargo.toml file

    launch(app)
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {

        header {
            nav { class: "bg-white border-gray-200 px-4 lg:px-6 py-2.5 dark:bg-gray-800",

                div { class: "flex flex-wrap justify-between items-center mx-auto max-w-screen-xl",

                    a {
                        class: "flex items-center",
                        href: "",

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",
                            class: "mr-3 h-6 sm:h-9",

                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9"
                            }
                        }


                        span { class: "self-center text-xl font-semibold whitespace-nowrap dark:text-white",
                            "DakSud"
                        }
                    }
                    div { class: "flex items-center lg:order-2",

                        a {
                            href: "#",
                            class: "text-gray-800 dark:text-white hover:bg-gray-50 focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-4 lg:px-5 py-2 lg:py-2.5 mr-2 dark:hover:bg-gray-700 focus:outline-none dark:focus:ring-gray-800",
                            "Log in"
                        }

                        a {
                            href: "#",
                            class: "text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 lg:px-5 py-2 lg:py-2.5 mr-2 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800",
                            "Get started"
                        }

                        button {
                            //data-collapse-toggle: "mobile-menu-2",
                            //type: "button",
                            class: "inline-flex items-center p-2 ml-1 text-sm text-gray-500 rounded-lg lg:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
                            aria_controls: "mobile-menu-2",
                            aria_expanded: "false",

                            span { class: "sr-only", "Open main menu" }

                            svg {

                                class: "w-6 h-6",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                xmlns: "http://www.w3.org/2000/svg",

                                path {
                                    fill_rule: "evenodd",
                                    d: "M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z",
                                    clip_rule: "evenodd"
                                }
                            }

                            svg {
                                class: "hidden w-6 h-6",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                xmlns: "http://www.w3.org/2000/svg",

                                path {
                                    fill_rule: "evenodd",
                                    d: "M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z",
                                    clip_rule: "evenodd"
                                }
                            }
                        }
                    }

                    div {
                        class: "hidden justify-between items-center w-full lg:flex lg:w-auto lg:order-1",
                        id: "mobile-menu-2",

                        ul { class: "flex flex-col mt-4 font-medium lg:flex-row lg:space-x-8 lg:mt-0",

                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-white rounded bg-primary-700 lg:bg-transparent lg:text-primary-700 lg:p-0 dark:text-white",
                                    aria_current: "page",
                                    "Home"
                                }
                            }

                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Company"
                                }
                            }

                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Marketplace"
                                }
                            }

                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Features"
                                }
                            }

                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Team"
                                }
                            }

                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Contact"
                                }
                            }
                        }
                    }
                }
            }
        }

        div {
            class: "bg-cover backdrop-brightness-0",
            style: "background-image: url('/bg.png')",

            div { class: "h-full w-full backdrop-blur-sm relative isolate px-6 pt-14",

                div {
                    class: "absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80 ",
                    aria_hidden: "true",

                    div {
                        class: "relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]",
                        style: "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"
                    }
                }

                div { class: "mx-auto max-w-2xl py-32 sm:py-48 lg:py-56",

                    div { class: "hidden sm:mb-8 sm:flex sm:justify-center",

                        div { class: "relative rounded-full px-3 py-1 text-sm leading-6 text-white ring-1 ring-orange-500/10 hover:ring-orange-500/20",
                            "Announcing our next round of funding."

                            a {
                                href: "#",
                                class: "font-semibold text-orange-600",

                                span {
                                    class: "absolute inset-0",
                                    aria_hidden: "true"
                                }

                                " Read more"

                                span { aria_hidden: "true" }
                            }
                        }
                    }

                    div { class: "text-center",

                        h1 { class: "text-4xl font-bold tracking-tight text-white sm:text-6xl",
                            "Your gate to metal construction supplies at your service"
                        }

                        div { class: "mt-10 flex items-center justify-center gap-x-6",

                            a {
                                href: "#",
                                class: "rounded-md bg-blue-900 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-900",
                                "Get started"
                            }

                            a {
                                href: "#",
                                class: "text-sm font-semibold leading-6 text-orange-600",
                                "Learn more"

                                span { aria_hidden: "true", "→" }
                            }
                        }
                    }
                }

                div {
                    class: "absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]",
                    aria_hidden: "true",

                    div {
                        class: "relative left-[calc(50%+3rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%+36rem)] sm:w-[72.1875rem]",
                        style: "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"
                    }
                }
            }
        }

        div { class: "bg-white py-24 sm:py-32",

            div { class: "mx-auto max-w-2xl lg:text-center",

                h2 { class: "text-base font-semibold leading-7 text-indigo-600", "Build Safely" }

                p { class: "mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl",
                    "The Quality you need"
                }

                p { class: "mt-6 text-lg leading-8 text-gray-600",
                    "Quis tellus eget adipiscing convallis sit sit eget aliquet quis. Suspendisse eget egestas a elementum pulvinar et feugiat blandit at. In mi viverra elit nunc."
                }
            }

            div { class: "mx-auto mt-16 max-w-2xl sm:mt-24 lg:max-w-4xl",

                dl { class: "grid max-w-xl grid-cols-1 gap-x-8 gap-y-10 lg:max-w-none lg:grid-cols-2 lg:gap-y-16",

                    div { class: "relative pl-16",

                        dt { class: "text-base font-semibold leading-7 text-gray-900",

                            div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",

                                svg {
                                    class: "h-6 w-6 text-white",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    //aria_hidden: "true",

                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.233-2.33 3 3 0 013.758 3.848A3.752 3.752 0 0118 19.5H6.75z"
                                    }
                                }
                            }

                            "Title 00"
                        }

                        dd { class: "mt-2 text-base leading-7 text-gray-600",
                            "Morbi viverra dui mi arcu sed. Tellus semper adipiscing suspendisse semper morbi. Odio urna massa nunc massa."
                        }
                    }

                    div { class: "relative pl-16",

                        dt { class: "text-base font-semibold leading-7 text-gray-900",

                            div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",

                                svg {
                                    class: "h-6 w-6 text-white",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    stroke: "ccurrentColor",
                                    //aria_hidden: "true",

                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"
                                    }
                                }
                            }

                            "Title 01"
                        }

                        dd { class: "mt-2 text-base leading-7 text-gray-600",
                            "Sit quis amet rutrum tellus ullamcorper ultricies libero dolor eget. Sem sodales gravida quam turpis enim lacus amet."
                        }
                    }

                    div { class: "relative pl-16",

                        dt { class: "text-base font-semibold leading-7 text-gray-900",

                            div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",

                                svg {
                                    class: "h-6 w-6 text-white",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    //aria_hidden: "true",

                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
                                    }
                                }
                            }

                            "Title 02"
                        }

                        dd { class: "mt-2 text-base leading-7 text-gray-600",
                            "Quisque est vel vulputate cursus. Risus proin diam nunc commodo. Lobortis auctor congue commodo diam neque."
                        }
                    }

                    div { class: "relative pl-16",

                        dt { class: "text-base font-semibold leading-7 text-gray-900",

                            div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",

                                svg {
                                    class: "h-6 w-6 text-white",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    //aria_hidden: "true",

                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M7.864 4.243A7.5 7.5 0 0119.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 004.5 10.5a7.464 7.464 0 01-1.15 3.993m1.989 3.559A11.209 11.209 0 008.25 10.5a3.75 3.75 0 117.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 01-3.6 9.75m6.633-4.596a18.666 18.666 0 01-2.485 5.33"
                                    }
                                }
                            }

                            "Title 03"
                        }

                        dd { class: "mt-2 text-base leading-7 text-gray-600",

                            "Arcu egestas dolor vel iaculis in ipsum mauris. Tincidunt mattis aliquet hac quis. Id hac maecenas ac donec pharetra eget."
                        }
                    }
                }
            }
        }
    }

    /*rsx!(
        div {
            class: "h-screen w-full",

            div {
                class: "bg-local h-3/4",
                //style: "background-image: url('https://flowbite.com/docs/images/logo.svg')",
                style: "background-image: url('/bg.png')",

                img {
                    src: "https://flowbite.com/docs/images/logo.svg",
                    //src: "C:/Rust/dioxus-demo/bg.png",
                    class: "mr-3 h-6 sm:h-9",
                    alt: "Flowbite Logo"
                }
            }
        }
    )*/
}

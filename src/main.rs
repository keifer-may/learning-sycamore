use sycamore::prelude::*;
use sycamore::web::html::p;
use sycamore::builder::t;
use sycamore::builder::prelude::*;

fn main() {
    // sycamore::render(|cx| view! { cx,
    //     p { "Hello, World!" }
    // });

    // sycamore::render(|cx| view! {
    //     cx,
    //     div {}
    //     div(class="foo")
    //     p {}
    //     my-custom-element {}
    // });

    // sycamore::render(
    //     |cx| view! {
    //         cx,
    //         "Hello World!"
    //     }
    // );

    sycamore::render(|cx| view!{
        cx,
        div {
            p{
                span{
                    "Hello "
                }
                strong{
                    "World!"
                }
            }
        }
    });


    ////////////////
    let my_number = 123;


    sycamore::render(|cx| 
        view!{ cx,
        p {
            "My number is: " (my_number)
        }
        h1{
            "Hello again, world!"
        }
    });
    //////////////////
    
    ///////////// This creates interpolated views within the same rendered page! ///////
    ///////took forever to figure out the cx/scope/ closure mess!//////
    create_scope(|cx|
        {
            let inner_view = view! 
            { cx,
                "Inside"
            };
            
            let outer_view = view! { cx,
                "Outside"
                div {
                    (inner_view)
                }
            };

            sycamore::render(|_| outer_view);
        }
        
    );
    //////////////////////////////////////

    sycamore::render(|cx| view! { cx,
        p(class="my-class", id="my-paragraph", aria-label="My paragraph"){
            "this my paragraph tag"
        }
        button(disabled=true) {
           "My button"
        }
    }
    );

    sycamore::render(|cx| view!{ 
        cx,
        input(type="checkbox", prop:indeterminate=false)
    });

    
    /////////////
    sycamore::render(|cx| view! {
        cx,
        button(on:click=|_| { /* do something */ }) {
            "Click me"
        }
    });
    



    sycamore::render(|cx| view! {
        cx,
        h2{
            "Let's explore some reactivity within Sycamore using signals!"
        }
    });

create_scope(|cx| {
    let state = create_signal(cx, 0); // Create a reactive atom with an initial value of `0`.
    println!("The state is: {}", state.get()); // prints "The state is: 0"
    state.set(1);
    println!("The state is: {}", state.get()); // should now print "The state is: 1"
});

    sycamore::render(|cx| {
        let state = create_signal(cx, 0);
        // view! ({ cx,
        //     p {
        //         "Our state is set to: "(state.get())
        //     }
        // });
        state.set(1);
        view! { cx,
            p {
                "Our state is set to: "(state.get())
            }
        }

    });



    ///////////////////////////////
   
    ///////////////////////////////

    


    

    
      







}


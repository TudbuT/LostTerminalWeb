use color_eyre::Report;
use yate::html;


fn template(inner: String) -> String {
    html! {

        <!DOCTYPE html>
        <html lang="en">

        <head>
            <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
            <link rel="icon" href="favicon.png"/>
            <link href="bulma-min.css" rel="stylesheet"/>
            <script src="https://kit.fontawesome.com/333f3de551.js" crossorigin="anonymous"></script>

            <meta charset="utf-8"/>
            <meta name="description" content="/usr/stories podcast"/>
            <meta content="width=device-width, initial-scale=1" name="viewport"/>
            <script src="https://kit.fontawesome.com/333f3de551.js" crossorigin="anonymous"></script>
            <title>"/usr/stories podcast"</title>
            <style>"
                .navbar-item, .navbar-link, .has-dropbown :hover {
                    background-color: #0a0a0a !important;
                }
                .navbar-link:not(.is-arrowless)::after {
                    border-color: white;
                }
                .navbar-burger {
                    color: white;
                }
                //.hero {
                //    background: #000 url(bg.jpg) center / cover;
                //}
            "</style>
        </head>


    <body class="is-family-monospace">
        <section class="hero"/>

            <nav class="navbar has-background-black" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">

                    <a role="button" class="navbar-burger burger has-color-white" aria-label="menu" aria-expanded="false" data-target="navbar">
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>

                <div id="navbar" class="navbar-menu has-background-black">
                    <div class="navbar-start">

/*
                        <a class="navbar-item has-text-white" href="store.html">

                            <span class="icon is-small">

                          <i class="fas fa-box"></i>
                        </span>
                          "&nbsp;Store"
                        </a>
*/
                        <a class="navbar-item has-text-white" href="https://twitter.com/usrstories">
                            <span class="icon is-small">
                          <i class="fab fa-twitter"></i>
                        </span>
                          "&nbsp;Follow on Twitter"
                        </a>

                        //<a class="navbar-item has-text-white" href="https://www.youtube.com/watch?v=Q3AhzHq8ogs&list=PLZaoyhMXgBzoM9bfb5pyUOT3zjnaDdSEP&index=1">
                        //  <span class="icon is-small">
                        //    <i class="fab fa-youtube"></i>
                        //  </span>
                        //  "&nbsp;Watch on YouTube"
                        //  </a>

                    <a class="navbar-item has-text-white" href="https://discord.gg/mCY2bBmDKZ">

                      <span class="icon is-small">
                        <i class="fab fa-discord"></i>
                      </span>
                      "&nbsp;Chat on Discord"
                    </a>


                    </div>
                    
                    //<div class="navbar-end">
                    //    <div class="navbar-item">
                    //        <div class="buttons">
                    //            <a class="button" href="https://www.patreon.com/noboilerplate">
                    //          <i class="fab fa-patreon"></i>
                    //      "&nbsp;Support me on Patreon"
                    //        </a>
                    //        </div>
                    //    </div>
                    //</div>
                </div>
            </nav>
            <section class="hero is-fullheight-with-navbar has-background-black has-text-light">
                <div class="hero-body">
                    <container class="container">
                        <div class="columns">
                            <div class="column is-three-fifths is-offset-one-fifth">

                            {inner}


                            </div>
                        </div>
                    </container>
                </div>
            </section>
        <footer class="footer has-text-white has-background-black">
        <div class="content has-text-centered">
            <p>
                    <a href="privacy.html">"Privacy Policy"</a>
            </p>
        </div>
        </footer>
    </body>

        <script language="javascript">
        r#"
    document.addEventListener('DOMContentLoaded', () => {

  // Get all "navbar-burger" elements
  const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);

  // Check if there are any navbar burgers
  if ($navbarBurgers.length > 0) {

    // Add a click event on each of them
    $navbarBurgers.forEach( el => {
      el.addEventListener('click', () => {

        // Get the target from the "data-target" attribute
        const target = el.dataset.target;
        const $target = document.getElementById(target);

        // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
        el.classList.toggle('is-active');
        $target.classList.toggle('is-active');

      });
    });
  }

      });
"#
</script>
        </html>
         }
}


fn index() -> String {
    template(html!{
        //<figure class="image">
        //    <img alt="logo" src="logo.png"/>
        //</figure>

        <span class="is-family-monospace is-size-4">
            "/usr/stories are fringe tales<br/> from the computer revolution."

        </span>

        <p class="has-text-center">
            <a href="https://twitter.com/usrstories" class="button has-text-right">
                "START HERE"
            </a>
        </p>
    })
}


fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    std::fs::write("docs/index.html", index())?;
    println!("Built site OK!");
    Ok(())
}

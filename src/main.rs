use color_eyre::Report;
use html_to_string_macro::html;

fn template(inner: String) -> String {
    let page_style = r#"
      body {
        font: 100% Courier New, monospace;
        margin: 20px;
        //line-height: 26px;
        background-color: #000000;
        padding: 10px;
        color: #84c754;
      }
      a {
        color: white;
      }
      .white {
        color: white;
      }
      .glow {
        color: #84c754;
        -webkit-animation: glow 1s linear infinite alternate;
        -moz-animation: glow 1s linear infinite alternate;
        animation: glow 1s linear infinite alternate;
        text-shadow: 0 0 5px #fff;
      }
      @-webkit-keyframes glow {
        from {
          text-shadow: 0 0 10px #fff;
        }
        to {
          text-shadow: 0 0 15px #fff;
        }
      }
      .narrow {
         max-width: 50%;
      }
      h1 {
        max-width: 40rem;
        line-height: 3rem;
      }"#;
    html! {
           <!DOCTYPE html>
           <html lang="en">

           <head>
               <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
               <link rel="icon" href="favicon.png"/>

               <meta charset="utf-8"/>
               <meta name="description" content="/usr/stories podcast"/>
               <meta content="width=device-width, initial-scale=1" name="viewport"/>
               <title>"/usr/stories podcast"</title>

               <style>

               {page_style}

               </style>

           </head>

    <body class="glow">
      <center>
        <a href="https://www.spreaker.com/show/lost-terminal">"Listen"</a>"&nbsp;&nbsp;"
        <a href="https://fosstodon.org/@lostterminal">"Follow me on Mastodon"</a>"&nbsp;&nbsp;"
        <a href="credits.html">"Credits"</a>"&nbsp;&nbsp;"
        <a href="https://discord.gg/mCY2bBmDKZ">"Chat on Discord"</a>"&nbsp;&nbsp;"
        <a href="https://www.patreon.com/lostterminalpod">"Support me on Patreon"</a>"&nbsp;&nbsp;"
        <br/>
        <br/>


        <img src="https://lostterminal.com/logo.png" width="50%"/>
           {inner}


    </center>
    </body>
     <script src="https://unpkg.com/typeit@8.7.1/dist/index.umd.js"></script>
     <script language="javascript">
     r##"
     new TypeIt("#typed", {
        speed: 1,
        loop: false,
        html: true,
        nextStringDelay: 0
      }).go();
      "##
     </script>

     </html>
    }
}

fn index() -> String {
    template(html! {

              <div class="white"> "How do you learn to be human if there's no-one around to teach you?" </div>
              <br/>
              <br/>
          "A hopepunk podcast following the journey of a little satellite trying to understand what has happened after Earth stops returning his calls."
      <div class="narrow">
      </div>
    <br/>
    <br/>

    <div id="typed">"New episodes Mondays." "&nbsp;"

        <a href="https://www.spreaker.com/show/lost-terminal">"Listen here"</a>
    </div>

    })
}

fn credits() -> String {
    template(html! {
        <br/>
        <br/>

    <span class="is-family-monospace">
      "Every episode has headline credits within the episode, and extended credits in the descriptions, both on youtube and in the shownotes of the podcast."
      <br/>
      "However, Lost Terminal would not be what it is today without these people, who I would like to thank here too:"
      <br/>
      <br/>
      <br/>



      <h2 class="title has-text-white">"Voices"</h2>

      <div class="list">
          <li>"Credits voiced by Lucy Stringer"</li>
          <li>"Antarctica voiced by Wolfie Thorns"</li>
          <li>"Ivan voiced by Alex Bayly"</li>
          <li>"Ana voiced by Oriel Winslow"</li>
          <li>"MINI voiced by Kate Ashford"</li>
          <li>"Yeshi voiced by Robin Howell"</li>
          <li>"PETER voiced by Karl Williams"</li>
          <li>"Alexander voiced by Dan Yilmaz"</li>
          <li>"50Meg voiced by Carin Calder-La Croix"</li>
          <li>"Alec voiced by Neil Murton"</li>
          <li>"NANA voiced by Lisa Ashton-Riemers"</li>
          <li>"Dr Redwing voiced by Gina Sneesby"</li>
      </div>


      <br/>
      <br/>

      <h2 class="title has-text-white">"Producers"</h2>

      <div class="list">
          <li>"Ada Phillips"</li>
          <li>"Kit"</li>
          <li>"Wil Taylor"</li>
          <li>"Kit"</li>
          <li>"deeryeen"</li>
          <li>"Andrew Krieg"</li>
          <li>"Toby"</li>
          <li>"Jade Felicity Bilkey"</li>
      </div>


      <br/>
      <br/>

      <h2 class="title has-text-white">"Character Concepts"</h2>

          "Sometimes friends suggest ideas to me, and sometimes I include them! (after some modification). If you are a patron, look" <a href="https://www.patreon.com/posts/52133498">"here"</a> "for how your character ideas can be included in Lost Terminal."
          <br/>
          <br/>

      <div class="list">
          <li>"EMMA and Dr Redwing created by Robert Bettelheim"</li>
          <li>"IVAN created by Karl Williams"</li>
          <li>"Yeshi created by Petra Bačkovská"</li>
          <li>"TASSI created by CM-47"</li>
          <li>"NANA created by Sal Boye"</li>
      </div>


      <br/>
      <br/>

      <h2 class="title has-text-white">"Additional Material"</h2>
      <div class="list">
          <li>"Podcast artwork by Carl Huber" <a class="has-text-white" href="http://www.carlh.com">"(carlh.com)"</a></li>
      </div>
    </span>


        })
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    let _ = build(vec![
        ("docs/index.html", index),
        ("docs/credits.html", credits),
    ]);
    println!("Built site OK!");
    Ok(())
}

fn build(pages: Vec<(&str, fn() -> String)>) -> Result<(), Report> {
    for (page, fun) in pages {
        std::fs::write(page, fun())?;
    }
    Ok(())
}

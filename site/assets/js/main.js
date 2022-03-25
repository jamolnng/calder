function change_theme() {
  logos = document.getElementsByClassName("logo");
  theme = document.getElementById("theme").value;
  load_style_sheet("/assets/themes/" + theme + "/css/custom.css", function (success, link) { }, this);
  for (const logo of logos) {
    logo.src = "/assets/themes/" + theme + "/logo.svg";
  }
}

function change_language(lang) {
  translatables = document.getElementsByClassName("translatable");
  data = Array(translatables.length);
  for (let i = 0; i < translatables.length; i++) {
    if (translatables[i].hasAttribute("translate-tag")) {
      data[i] = translatables[i].getAttribute("translate-tag");
    }
  }
  console.log(JSON.stringify({ langid: lang, values: data }));
  fetch("/locale", {
    method: 'POST',
    mode: 'no-cors',
    cache: 'no-cache',
    credentials: 'same-origin',
    headers: { 'Content-Type': 'application/json' },
    redirect: 'follow',
    referrerPolicy: 'no-referrer',
    body: JSON.stringify({ langid: lang, values: data })
  }).then(response => response.json())
    .then(response => {
      for (const t of translatables) {
        if (t.hasAttribute("translate-tag")) {
          tag = t.getAttribute("translate-tag");
          tagname = t.tagName.toLowerCase();
          if (tagname === "input") {
            t.setAttribute("placeholder", response[tag]);
          } else {
            t.firstChild.nodeValue = response[tag];
          }
        }
      }
    });
}

// function change_language(lang) {
//   change_language2(lang);
//   return;
//   translatables = document.getElementsByClassName("translatable");
//   for (const t of translatables) {
//     if (t.hasAttribute("translate-tag")) {
//       tag = t.getAttribute("translate-tag");
//       fetch("/locale?code=" + lang + "&tag=" + tag)
//         .then(response => response.text())
//         .then(new_json => {
//           tagname = t.tagName.toLowerCase();
//           if (tagname === "input") {
//             t.setAttribute("placeholder", new_json);
//           } else {
//             t.firstChild.nodeValue = new_json;
//           }
//         });
//     }
//   }
// }

// function change_language(lang) {
//   fetch("/locale?en_US")
//     .then(response => response.json())
//     .then(en_json => {
//       fetch("/locale?" + lang)
//         .then(response => response.json())
//         .then(new_json => {
//           combined = { "name": new_json["name"], "map": { ...en_json["map"], ...new_json["map"] } };
//           translatables = document.getElementsByClassName("translatable");
//           for (const t of translatables) {
//             if (t.hasAttribute("translate-tag")) {
//               tag = t.getAttribute("translate-tag");
//               tagname = t.tagName.toLowerCase();
//               if (tagname === "input") {
//                 t.setAttribute("placeholder", combined["map"][tag]);
//               } else {
//                 t.firstChild.nodeValue = combined["map"][tag];
//               }
//             }
//           }
//         });
//     });
// }

function load_style_sheet(path, fn, scope) {
  var head = document.getElementsByTagName('head')[0], // reference to document.head for appending/ removing link nodes
  link = document.createElement('link');               // create the link node
  link.setAttribute('href', path);
  link.setAttribute('rel', 'stylesheet');
  link.setAttribute('type', 'text/css');

  // var sheet, cssRules;
  // get the correct properties to check for depending on the browser
  // if ('sheet' in link) {
  //   sheet = 'sheet';
  //   cssRules = 'cssRules';
  // }
  // else {
  //   sheet = 'styleSheet';
  //   cssRules = 'rules';
  // }
  // 
  // var interval_id = setInterval(function () {            // start checking whether the style sheet has successfully loaded
  //   try {
  //     if (link[sheet] && link[sheet][cssRules].length) { // SUCCESS! our style sheet has loaded
  //       clearInterval(interval_id);                      // clear the counters
  //       clearTimeout(timeout_id);
  //       fn.call(scope || window, true, link);            // fire the callback with success == true
  //     }
  //   } catch (e) { } finally { }
  // }, 10),                                                // how often to check if the stylesheet is loaded
  //   timeout_id = setTimeout(function () {                // start counting down till fail
  //     clearInterval(interval_id);                        // clear the counters
  //     clearTimeout(timeout_id);
  //     head.removeChild(link);                            // since the style sheet didn't load, remove the link node from the DOM
  //     fn.call(scope || window, false, link);             // fire the callback with success == false
  //   }, 15000);                                           // how long to wait before failing

  head.appendChild(link);                                   // insert the link node into the DOM and start loading the style sheet

  return link;                                              // return the link node;
}

function init_id_good(id) {
  document.getElementById(id).classList.remove("error");
  document.getElementById(id).classList.remove("apply-shake");
  document.getElementById(id).addEventListener('animationend', () => {
    document.getElementById(id).classList.remove("apply-shake");
  });
}

function set_id_err(id) {
  document.getElementById(id).classList.add("error");
  document.getElementById(id).classList.add("apply-shake");
}

function verify_setup() {
  var username = document.forms["setup"]["username"].value;
  var password = document.forms["setup"]["password"].value;
  var confirm = document.forms["setup"]["confirm_password"].value;
  var server_name = document.forms["setup"]["server_name"].value;
  var ret = true;

  init_id_good("username");
  init_id_good("password");
  init_id_good("confirm_password");
  init_id_good("server_name");

  if (username.length === 0) {
    ret = false;
    set_id_err("username");
  }
  if (password.length < 8) {
    ret = false;
    set_id_err("password");
  }
  if (password !== confirm) {
    ret = false;
    set_id_err("password");
    set_id_err("confirm_password");
  }
  if (server_name.length === 0) {
    ret = false;
    set_id_err("server_name");
  }
  return ret;
}

function verify_login() {
  var username = document.forms["login"]["username"].value;
  var password = document.forms["login"]["password"].value;
  var ret = true;

  init_id_good("username");
  init_id_good("password");

  if (username.length === 0) {
    ret = false;
    set_id_err("username");
  }
  if (password.length == 0) {
    ret = false;
    set_id_err("password");
  }
  return ret;
}
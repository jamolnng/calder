hljs.highlightAll();

hljs.initLineNumbersOnLoad();

const prefersDarkScheme = window.matchMedia("(prefers-color-scheme: dark)");

window.onload = function () {
  var theme = localStorage.getItem("theme");
  if (theme === null) {
    if (prefersDarkScheme.matches) {
      load_theme("dark");
    } else {
      load_theme("light");
    }
  } else {
    load_theme(theme);
  }
};

function theme_slider(slider) {
  if (slider.checked) {
    load_theme("light");
  } else {
    load_theme("dark");
  }
}

function load_theme(theme) {
  localStorage.setItem("theme", theme);
  if (theme === "light") {
    document.getElementById("themetoggle-checkbox").checked = true;
  } else {
    document.getElementById("themetoggle-checkbox").checked = false;
  }
  load_style_sheet("/assets/" + theme + ".css");
}

function load_style_sheet(path) {
  var head = document.getElementsByTagName('head')[0];
  link = document.createElement('link');
  link.setAttribute('href', path);
  link.setAttribute('rel', 'stylesheet');
  link.setAttribute('type', 'text/css');

  head.appendChild(link);
  return link;
}
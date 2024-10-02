pub async fn test_js() -> String {
  let get_script = get_script().await;
  let get_css = get_css().await;
    "
        <!DOCTYPE html>
<html lang=\"en\" dir=\"ltr\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\" />
    <meta http-equiv=\"cache-control\" content=\"max-age=0\" />
    <meta http-equiv=\"cache-control\" content=\"no-cache\" />
    <meta http-equiv=\"expires\" content=\"0\" />
    <meta http-equiv=\"expires\" content=\"Tue, 01 Jan 1980 1:00:00 GMT\" />
    <meta http-equiv=\"pragma\" content=\"no-cache\" />
    <title>Animated Sidebar Menu | CodingLab</title>
    <style>".to_string() + &get_css + &" </style>
  </head>
  <body>

    <div class=\"grid_water\">
      <div class=\"water_left\">
        <h4>Холодная вода была</h4>
        <input id=\"num3\" />
      </div>
      <div class=\"border-right\"></div>
      <div class=\"border-right-second\"></div>
      <div class=\"border-right-third\"></div>
      <div class=\"water-cold-now\">
        <h4>Холодная вода стало</h4>
        <input id=\"num4\" />
      </div>
      <div class=\"cold-water\">
        <h4>Холодная вода</h4>
        <p id=\"cold_water\">куб</p>
      </div>
      <div class=\"hot-water-before\">
        <h4>Горячая вода была</h4>
        <input id=\"num5\" />
      </div>
      <div class=\"hot-water-now\">
        <h4>Горячая вода стало</h4>
        <input id=\"num6\" />
      </div>
      <div class=\"hot-water\">
        <h4>Горячая вода</h4>
        <p id=\"hot_water\">куб</p>
      </div>
      <div class=\"water_sum_cub\">
        <h4>Сумма кубов воды</h4>
        <p id=\"all_water\">куб</p>
      </div>
    </div>

    <div class=\"grid_money_water\">
      <div class=\"tarif-water\">
        <h4>Тариф вода</h4>
        <input id=\"num7\" />
      </div>
      <div class=\"tarif-water-out\">
        <h4>Тариф водоотведение</h4>
        <input id=\"water_out\" />
      </div>
      <div class=\"money_water_left\">
        <h4>Денег за воду</h4>
        <b><p id=\"money_water\">₽</p></b>
      </div>
      <div class=\"money_water_out\">
        <h4>Денег за водоотведение</h4>
        <b><p id=\"money_water_out\">₽</p></b>
      </div>
      <div class=\"money_water\">
        <h4>Сумма за воду и водоотведение</h4>
        <b><p id=\"sum_water_and_water_out\">₽</p></b>
      </div>
    </div>

    <div class=\"tko_div\">
      <div class=\"tko_center\">
        <h4>Обращение с ТКО</h4>
        <input id=\"tko\" />
      </div>
    </div>

    <div class=\"el_all\">
      <div class=\"el\">
        <h3>Электричество было</h3>
        <input id=\"el_before\" />
      </div>
      <div class=\"el\">
        <h3>Электричество стало</h3>
        <input id=\"el_now\" />
      </div>
      <div class=\"el\">
        <h3>Тариф Электричество</h3>
        <input id=\"tarif_el\" />
      </div>
      <div class=\"el\">
        <div class=\"el_month\">
          <h4>Электричество за месяц</h4>
          <p id=\"el_sum\">Кв</p>
        </div>
      </div>
      <div class=\"el\">
        <h4>Денег за Электричество</h4>
        <b><p id=\"money_el\">₽</p></b>
      </div>
    </div>

    <br />
    <br />

      <button class=\"btn\" onclick=\"func()\">Посчитать!</button>
    <div class=\"money_alll\">
      <h4>Денег за Всё</h4>
      <b><p id=\"money_all\">₽</p></b>
    </div>
    <input type=\"checkbox\" id=\"input_theme\" name=\"checkbox\" />
    <label for=\"input_theme\" class=\"label_theme\">
      <img
        src=\"./src/view/svg/brightness.svg\"
        class=\"light_check\"
        alt=\"За стеклом\"
        width=\"24\"
      />
      <img
        src=\"./src/view/svg/moon-stars.svg\"
        class=\"dark_check\"
        alt=\"За стеклом\"
        width=\"24\"
      />
    </label>
 <script> ".to_string() + &get_script + &" </script>
   </body>
</html>
      ".to_string()
}
async fn get_script() -> String {
  "
var op;

function func() {
  var result;
  var cold_before = Number(document.getElementById(\"num3\").value);
  var cold_now = Number(document.getElementById(\"num4\").value);
  var hot_water_before = Number(document.getElementById(\"num5\").value);
  var hot_water_now = Number(document.getElementById(\"num6\").value);
  var tarif_water = Number(document.getElementById(\"num7\").value);
  var tko = Number(document.getElementById(\"tko\").value);
  var el_before = Number(document.getElementById(\"el_before\").value);
  var el_now = Number(document.getElementById(\"el_now\").value);
  var tarif_el = Number(document.getElementById(\"tarif_el\").value);
  var water_out = Number(document.getElementById(\"water_out\").value);
  var sum_water_and_water_out = Number(document.getElementById(\"sum_water_and_water_out\").value);

  var cold = cold_now - cold_before;
  cold = Number(cold.toFixed(2));
  const show_cold = cold + \" куб\";
  var hot = hot_water_now - hot_water_before;
  hot = Number(hot.toFixed(2));
  const show_hot = hot + \" куб\";
  var sum_water = cold + hot;
  sum_water = Number(sum_water.toFixed(2));
  const show_sum_water = sum_water + \" куб\";
  var money_water = tarif_water * sum_water;
  money_water = Number(money_water.toFixed(2));
  var show_money_water = money_water + ' ₽';
  var el_sum = el_now - el_before;
  el_sum = Number(el_sum.toFixed(2));
  const show_el_sum = el_sum + \" Кв\";
  var el_money = tarif_el * el_sum;
  el_money = Number(el_money.toFixed(2));
  var show_el_money = el_money + ' ₽';
  var money_water_out = water_out * sum_water;
  money_water_out = Number(money_water_out.toFixed(2));
  var show_money_water_out = money_water_out + \" ₽\";
  var money_water_water_out = money_water_out + money_water;
  money_water_water_out = Number(money_water_water_out.toFixed(2));
  var show_money_water_water_out = money_water_water_out + \" ₽\";

  var money_all = money_water + money_water_out + tko + el_money;
  money_all = Number(money_all.toFixed(2));
  show_money_all = money_all + \" ₽\";


  document.getElementById(\"cold_water\").innerHTML = show_cold;
  document.getElementById(\"hot_water\").innerHTML = show_hot;
  document.getElementById(\"all_water\").innerHTML = show_sum_water;
  document.getElementById(\"money_water\").innerHTML = show_money_water;
  document.getElementById(\"money_water_out\").innerHTML = show_money_water_out;
  document.getElementById(\"sum_water_and_water_out\").innerHTML = show_money_water_water_out;
  document.getElementById(\"el_sum\").innerHTML = show_el_sum;
  document.getElementById(\"money_el\").innerHTML = show_el_money;
  document.getElementById(\"money_all\").innerHTML = show_money_all;
}

const darkThemeClass = \"dark-theme\";
const lightThemeClass = \"light-theme\";

const initTheme = () => {
  const isDark =
    window.matchMedia &&
    window.matchMedia(\"(prefers-color-scheme: dark)\").matches;
  console.log(isDark);

if (isDark) {
  document.getElementById(\"input_theme\").checked = true;
  document.body.classList.add(darkThemeClass);
  document.body.classList.remove(lightThemeClass);
} else {
  document.getElementById(\"input_theme\").checked = false;
  document.body.classList.add(lightThemeClass);
  document.body.classList.remove(darkThemeClass);
}

};
var checkbox = document.querySelector(\"input[id=input_theme]\");

checkbox.addEventListener(\"change\", function () {
  if (this.checked) {
    document.body.classList.add(darkThemeClass);
    document.body.classList.remove(lightThemeClass);
  } else {
    document.body.classList.add(lightThemeClass);
    document.body.classList.remove(darkThemeClass);
  }
});

function check() {
  document.getElementById(\"input_theme\").checked = true;
  console.log(\"check\");
}

initTheme();
  ".to_string()
}
async fn get_css() -> String {
  "
    * {
  margin: 0;
  padding: 0;
  font-family: \"poppins\", sans-serif;
}
body.light-theme {
  --color-input-backgrund: #ffffff;
  --color-text: #2e353f;
  --color-background: #fff;
  --color-fone-background: #eee;
  --color-border: #1d5831;
  --color-filter: brightness(0) saturate(100%) invert(10%) sepia(11%)
    saturate(1275%) hue-rotate(201deg) brightness(97%) contrast(90%);
}
body.dark-theme {
  --color-input-backgrund: #a3a3a3;
  --color-text: #eee;
  --color-background: #1b1b1b;
  --color-fone-background: #23232e;
  --color-border: #1d5831;
  --color-filter: brightness(0) saturate(100%) invert(100%) sepia(0%)
    saturate(2149%) hue-rotate(310deg) brightness(117%) contrast(87%);
}
body {
  z-index: 100;
  background-color: var(--color-background);
  color: var(--color-text);
}
@media (min-width: 40rem) {
  .grid_water {
    position: absolute;
    margin-left: 2rem;
    margin-top: 2rem;
    padding: 0rem 0rem 0rem 0rem;
    display: grid;
    grid-template-columns: 16rem 18rem 13rem 15rem;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 30px;
    background-color: var(--color-fone-background);
  }
  #num3,
  #num4,
  #num5,
  #num6,
  #num7,
  #water_out,
  #tko,
  #el_before,
  #el_now,
  #tarif_el {
    background: var(--color-input-backgrund);
    border: 0.1rem solid var(--color-background);
    font-size: large;
    width: 8rem;
    border-radius: 10px;
    padding-left: 0.5rem;
    padding-top: 0.1rem;
    padding-bottom: 0.1rem;
    padding-right: 0.6rem;
  }
  #tko,
  #money_water,
  #money_water_out,
  #sum_water_and_water_out,
  #num3,
  #num4,
  #num5,
  #num6,
  #num7,
  #water_out,
  #el_before,
  #el_now,
  #tarif_el,
  #el_sum,
  #money_el,
  #money_all,
  #cold_water {
    margin-top: 1rem;
  }
  #hot_water,
  #all_water {
    padding-top: 1rem;
  }
  .water {
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .water_sum_cub {
    grid-column-start: 4;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .water-cold-now {
    grid-column-start: 1;
    grid-row-start: 2;
    border-right: 1px solid var(--color-border);
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .hot-water-before {
    grid-column-start: 2;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
    border-right: 1px solid var(--color-border);
  }
  .hot-water-now {
    padding-left: 3rem;
    padding-top: 1rem;
    border-right: 1px solid var(--color-border);
  }
  .border-right {
    border-right: 1px solid var(--color-border);
    padding-top: 2rem;
    grid-column-start: 1;
    grid-row-start: 3;
  }
  .border-right-second {
    border-right: 1px solid var(--color-border);
    padding-top: 2rem;
    grid-column-start: 2;
    grid-row-start: 3;
  }
  .border-right-third {
    border-right: 1px solid var(--color-border);
    padding-top: 2rem;
    grid-row-start: 3;
    grid-column-start: 3;
  }
  .hot-water {
    border-right: 1px solid var(--color-border);
    grid-column-start: 3;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .cold-water {
    border-right: 1px solid var(--color-border);
    grid-row-start: 1;
    grid-column-start: 3;
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .water_left {
    border-right: 1px solid var(--color-border);
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .money_water {
    grid-column-start: 3;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
    border-top: 1px solid var(--color-border);
  }

  .money_water_left {
    grid-column-start: 1;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
    border-top: 1px solid var(--color-border);
  }
  .money_water_out {
    grid-column-start: 2;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
    border-top: 1px solid var(--color-border);
  }
  .tarif-water {
    grid-column-start: 1;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
    padding-bottom: 1rem;
  }
  .tarif-water-out {
    grid-column-start: 2;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
    padding-bottom: 1rem;
  }
  .result-money {
    position: absolute;
    display: grid;
    margin-top: 35rem;
    margin-left: 3rem;
    grid-template-columns: 12rem 8rem;
  }
  .money_alll {
    margin-top: 40rem;
    margin-left: 3rem;
  }
  .btn {
    position: absolute;
    width: 10rem;
    height: 4rem;
    color: rgb(230, 230, 230);
    margin-top: 35rem;
    margin-left: 3rem;
    border: none;
    padding: 0.7rem 2rem;
    border-radius: 0.6rem;
    background-color: var(--color-border);
    transition: 0.2s; 
  }
  .money-div-all {
    grid-column-start: 1;
    grid-row-start: 2;
  }
  .grid_money_water {
    position: absolute;
    display: grid;
    margin-top: 15rem;
    margin-left: 2rem;
    padding: 0rem 0rem 1rem 0rem;
    grid-template-columns: 16rem 18rem 22rem;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 30px;
    background-color: var(--color-fone-background);
  }
  .tko_div {
    position: absolute;
    padding: 1.5rem 1.5rem 1.5rem 1.5rem;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 29px;
    background-color: var(--color-fone-background);
    margin-top: 17rem;
    margin-left: 60rem;
  }
  .el_all {
    position: absolute;
    display: grid;
    padding: 1.5rem 1.5rem 1.5rem 1.5rem;
    grid-template-columns: 15rem 15rem 15rem 15rem 15rem;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 30px;
    background-color: var(--color-fone-background);
    margin-top: 28rem;
    margin-left: 2rem;
  }
  #tko,
  #money_water,
  #money_water_out,
  #sum_water_and_water_out,
  #num3,
  #num4,
  #num5,
  #num6,
  #num7,
  #water_out,
  #el_before,
  #el_now,
  #tarif_el,
  #el_sum,
  #money_el,
  #cold_water {
    margin-top: 1rem;
  }
  #hot_water,
  #all_water {
    padding-top: 1rem;
  }
  .water {
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .water_sum_cub {
    grid-column-start: 4;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .water-cold-now {
    grid-column-start: 1;
    grid-row-start: 2;
    border-right: 1px solid var(--color-border);
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .hot-water-before {
    grid-column-start: 2;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
    border-right: 1px solid var(--color-border);
  }
  .hot-water-now {
    padding-left: 3rem;
    padding-top: 1rem;
    border-right: 1px solid var(--color-border);
  }
  .border-right {
    border-right: 1px solid var(--color-border);
    padding-top: 2rem;
    grid-column-start: 1;
    grid-row-start: 3;
  }
  .border-right-second {
    border-right: 1px solid var(--color-border);
    padding-top: 2rem;
    grid-column-start: 2;
    grid-row-start: 3;
  }
  .border-right-third {
    border-right: 1px solid var(--color-border);
    padding-top: 2rem;
    grid-row-start: 3;
    grid-column-start: 3;
  }
  .hot-water {
    border-right: 1px solid var(--color-border);
    grid-column-start: 3;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .cold-water {
    border-right: 1px solid var(--color-border);
    grid-row-start: 1;
    grid-column-start: 3;
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .water_left {
    border-right: 1px solid var(--color-border);
    padding-left: 3rem;
    padding-top: 1rem;
  }
  .money_water {
    grid-column-start: 3;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
    border-top: 1px solid var(--color-border);
  }

  .money_water_left {
    grid-column-start: 1;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
    border-top: 1px solid var(--color-border);
  }
  .money_water_out {
    grid-column-start: 2;
    grid-row-start: 2;
    padding-left: 3rem;
    padding-top: 1rem;
    border-top: 1px solid var(--color-border);
  }
  .tarif-water {
    grid-column-start: 1;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
    padding-bottom: 1rem;
  }
  .tarif-water-out {
    grid-column-start: 2;
    grid-row-start: 1;
    padding-left: 3rem;
    padding-top: 1rem;
    padding-bottom: 1rem;
  }
  .result_money {
    display: grid;
  }

  .light_check {
    position: absolute;
    bottom: 0;
    opacity: 0;
    margin: 0 0 1.7rem 1.7rem;
    transition: all 0.3s ease;
    filter: var(--color-filter);
  }
  .dark_check {
    position: absolute;
    bottom: 0;
    transition: all 0.3s ease;
    margin: 0 0 1.7rem 1.7rem;
    filter: var(--color-filter);
  }
  #input_theme {
    display: none;
  }
  #input_theme:checked ~ .label_theme .light_check {
    opacity: 1;
    transform: rotate(180deg);
    transition: all 0.3s ease;
  }
  #input_theme:checked ~ .label_theme .dark_check {
    opacity: 0;
  }
  #input_theme {
    position: absolute;
    bottom: 0;
    display: none;
  }

  .light_check {
    position: absolute;
    bottom: 0;
    opacity: 0;
    margin: 0 0 1.7rem 1.7rem;
    transition: all 0.3s ease;
    filter: var(--color-filter);
  }
  .dark_check {
    position: absolute;
    bottom: 0;
    transition: all 0.3s ease;
    margin: 0 0 1.7rem 1.7rem;
    filter: var(--color-filter);
  }
  #input_theme {
    display: none;
  }
  #input_theme:checked ~ .label_theme .light_check {
    opacity: 1;
    transform: rotate(180deg);
    transition: all 0.3s ease;
  }
  #input_theme:checked ~ .label_theme .dark_check {
    opacity: 0;
  }
  #input_theme {
    position: absolute;
    bottom: 0;
    display: none;
  }
}

@media (max-width: 40rem) {
  .grid_water {
    display: flexbox;
    margin-left: 1rem;
    margin-top: 1rem;
    margin-right: 1rem;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    border: 1px solid gray;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 30px;
    background-color: var(--color-fone-background);
  }
  .hot-water-now {
    margin-bottom: 1rem;
  }
  .grid_money_water {
    display: flexbox;
    margin-left: 1rem;
    margin-top: 1rem;
    margin-right: 1rem;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    border: 1px solid gray;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 30px;
    background-color: var(--color-fone-background);
  }
  #num3,
  #num4,
  #num5,
  #num6,
  #num7,
  #water_out,
  #tko,
  #el_before,
  #el_now,
  #tarif_el {
    background: var(--color-input-backgrund);
    border: 0.1rem solid var(--color-background);
    font-size: large;
    width: 8rem;
    border-radius: 10px;
    padding-left: 0.5rem;
    padding-top: 0.1rem;
    padding-bottom: 0.1rem;
    padding-right: 0.6rem;
  }
  .money_water {
    margin-bottom: 1rem;
  }
  .tko_div {
    display: flexbox;
    margin-left: 1rem;
    margin-top: 1rem;
    margin-right: 1rem;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    border: 1px solid gray;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 30px;
    background-color: var(--color-fone-background);
  }
  .tko_center {
    margin-top: 1rem;
    margin-bottom: 1rem;
    text-align: center;
  }
  #tko {
    margin-top: 1rem;
  }
  .el_all {
    display: flexbox;
    margin-left: 1rem;
    margin-top: 1rem;
    margin-right: 1rem;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    border: 1px solid gray;
    border-top: 0.1rem solid var(--color-border);
    border-left: 0.1rem solid var(--color-border);
    border-right: 0.3rem solid var(--color-border);
    border-bottom: 0.3rem solid var(--color-border);
    border-radius: 30px;
    background-color: var(--color-fone-background);
  }
  .el {
    margin-top: 1rem;
    text-align: center;
  }
  #money_el {
    margin-bottom: 1rem;
  }
  .cold-water,
  .hot-water,
  .water_sum_cub,
  .el_month {
    display: none;
  }
  .water_left,
  .water-cold-now,
  .hot-water-before,
  .hot-water-now {
    margin-top: 1rem;
    text-align: center;
  }
  .tarif-water,
  .tarif-water-out,
  .money_water_left,
  .money_water_out,
  .money_water {
    margin-top: 1rem;
    text-align: center;
  }
  #num3,
  #num4,
  #num5,
  #num6,
  #num7,
  #water_out,
  #tko,
  #el_before,
  #el_now,
  #tarif_el {
    display: block;
    margin-left: auto;
    margin-right: auto;
  }
  .btn {
    width: 10rem;
    height: 4rem;
    color: rgb(230, 230, 230);
    border: none;
    padding: 0.7rem 2rem;
    border-radius: 0.6rem;
    background-color: var(--color-border);
    transition: 0.2s;
    margin: 0 auto;
    display: block;
  }
  .money_alll {
    margin-top: 1rem;
    text-align: center;
  }

  .light_check {
    transition: all 0.3s ease;
    filter: var(--color-filter);
  }
  .dark_check {
    bottom: 0; 
    left: 0;
    margin-left: 0;
    transition: all 0.3s ease;
    filter: var(--color-filter);
  }
  #input_theme {
    display: none;
  }
  #input_theme:checked ~ .label_theme .light_check {
    opacity: 1;
    transform: rotate(180deg);
    transition: all 0.3s ease;
  }
  #input_theme:checked ~ .label_theme .dark_check {
    opacity: 0;
  }
  #input_theme {
    display: none;
    bottom: 0; 
    left: 0;
  }

  .light_check {
    bottom: 0; 
    left: 0;
    opacity: 0;
    transition: all 0.3s ease;
    filter: var(--color-filter);
  }
  .dark_check {
    bottom: 0; 
    left: 0;
    transition: all 0.3s ease;
    filter: var(--color-filter);
  }
  #input_theme {
    display: none;
  }
  #input_theme:checked ~ .label_theme .light_check {
    opacity: 1;
    bottom: 0; 
    left: 0;
    transform: rotate(180deg);
    transition: all 0.3s ease;
  }
  #input_theme:checked ~ .label_theme .dark_check {
    opacity: 0;
    bottom: 0; 
    left: 0;
  }
  #input_theme {
    bottom: 0; 
    left: 0;
    display: none;
  }
}
  ".to_string()
}

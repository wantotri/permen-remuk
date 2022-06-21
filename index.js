import * as wasm from './pkg';

let permenTerpilih = [];
let score = 0;

let petunjukBtn = document.getElementById("petunjuk");
petunjukBtn.addEventListener("click", e => {
  e.preventDefault();
  let vk = wasm.cekKemungkinan().trim().split(/\s+/).map(p => p.split(/:/).map(x => parseInt(x)));
  if (vk.length > 0) {
    vk.forEach(pos => {
      let permen = document.getElementById(`permen${pos[0]}${pos[1]}`);
      permen.classList.remove("petunjuk", "wiggle");
      void permen.offsetWidth;  // trigger a DOM reflow
      permen.classList.add("petunjuk", "wiggle");
    })
  }
})

async function main() {
  renderPapan(wasm.buatPapan());
}

function renderPapan(state) {
  let scoreCounter = document.getElementById("score");
  let app = document.getElementById("app");
  app.innerHTML = '';

  let data = state.split("\n").map((row) => row.trim().split(/\s+/));
  data.pop();  // remove the last row because it is empty

  app.style.display = "grid";
  app.style.gridTemplate = `repeat(${data.length}, auto) / repeat(${data[0].length}, auto)`;

  data.forEach((row, y) => {
    row.forEach((cell, x) => {
      let elm = document.createElement("a");
      elm.setAttribute("id", `permen${y}${x}`);
      elm.classList.add("permen", "m-1", "btn");

      let childElm = document.createElement("i");
      childElm.style.color = "white";

      switch (cell) {
        case "🟥":
          elm.style.backgroundColor = "red";
          childElm.classList.add("fas", "fa-hamburger");
          break;
        case "🟧":
          elm.style.backgroundColor = "orange";
          childElm.classList.add("fas", "fa-cheese");
          break;
        case "🟨":
          elm.style.backgroundColor = "hotpink";
          childElm.classList.add("fas", "fa-apple-alt");
          break;
        case "🟩":
          elm.style.backgroundColor = "limegreen";
          childElm.classList.add("fas", "fa-pizza-slice");
          break;
        case "🟦":
          elm.style.backgroundColor = "deepskyblue";
          childElm.classList.add("fas", "fa-beer");
          break;
        case "🟫":
          elm.style.backgroundColor = "brown";
          childElm.classList.add("fas", "fa-cookie");
          break;
        case "🟪":
          elm.style.backgroundColor = "purple";
          childElm.classList.add("fas", "fa-ice-cream");
          break;

        default:
          elm.style.backgroundColor = "white";
          childElm.classList.add("fas", "fa-times");
          break;
      }

      elm.append(childElm);

      elm.addEventListener("click", e => {
        e.preventDefault();

        let pos = `${y} ${x}`;
        // Menambahkan posisi ke permen terpilih
        if (permenTerpilih.indexOf(pos) < 0 && permenTerpilih.length < 3) {
          if (permenTerpilih.length == 1) {
            let [a, b] = permenTerpilih[0].split(" ").map(p => parseInt(p));
            if (Math.abs((a+b)-(y+x)) == 1) {
              permenTerpilih.push(pos);
              elm.classList.add("glow");
            }
          } else if (permenTerpilih.length == 0) {
            permenTerpilih.push(pos);
            elm.classList.add("glow");
          }
        } else {
          let idx = permenTerpilih.indexOf(pos);
          if (idx > -1) { permenTerpilih.splice(idx, 1) }
          elm.classList.remove("glow");
        }

        // Tukar posisi dan render kembali papan
        if (permenTerpilih.length == 2) {
          let [y1, x1] = permenTerpilih[0].split(" ").map(p => parseInt(p));
          let [y2, x2] = permenTerpilih[1].split(" ").map(p => parseInt(p));
          wasm.tukarPosisi(y1, x1, y2, x2);
          renderPapan(wasm.buatPapan());
          wasm.cekDempet(y1, x1, 3);
          wasm.cekDempet(y2, x2, 3);
          permenTerpilih = [];

          if (wasm.panjangDempet() == 0) {
            wasm.tukarPosisi(y1, x1, y2, x2);
            renderPapan(wasm.buatPapan());
          }
        }

        if (wasm.panjangDempet() > 0) {
          score += wasm.panjangDempet();
          wasm.remukanPermen();
          renderPapan(wasm.buatPapan());
        }

      });

      app.appendChild(elm);
      scoreCounter.innerText = score;
    });
  });

  if (!wasm.bisaJalan()) {
    scoreCounter.innerText += "   ⛔ Game Over ⛔"
  }
}

main();

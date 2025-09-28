const svg = document.getElementById("svg");
const template = document.getElementById("Bruch");
const body = document.getElementById("body");

document.addEventListener("DOMContentLoaded", () => {

    /*let text = document.createElementNS("http://www.w3.org/2000/svg", "text")
    text.textContent = "bla"
    text.setAttribute("x", "100")
    text.setAttribute("y", "100")
    svg.appendChild(text)*/

    let Bruch1 = new Bruch(1, 2);

    Bruch1.anzeigen();
})

body.appendChild(document.getElementById("test").content)

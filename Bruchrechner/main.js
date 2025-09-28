const svg = document.getElementById("svg");
const Bruchdefs = document.getElementById("Bruchdefs");
const body = document.getElementById("body");
const userinputfeld = document.getElementById("userinputfeld");

document.addEventListener("DOMContentLoaded", () => {

    let Bruch1 = new Bruch(3, 10);

    Bruch1.anzeigen();
    Bruch1.alsBalkenDarstellen();
})


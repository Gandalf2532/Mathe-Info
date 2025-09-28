//nach jeder Methode wird this.anzeigen() ausgeführt. Dadurch wird aktualisiert, was der Bruch anzeigt. Muss das wirklich so?

class Bruch {
  constructor(zaehler, nenner, xposition = 100) { //xposition noch nicht vollständig implementiert
    this.zaehler = zaehler;
    this.nenner = nenner;
    this.xposition = xposition;
    let el = document.createElementNS("http://www.w3.org/2000/svg", "g");
    this.gruppe = svg.appendChild(el);
  }

  anzeigen() {
    let Elements = [];
    Array.from(Bruchdefs.children).forEach(node => Elements.push(node.cloneNode(true)));

    let texts = Elements.filter(node => node.tagName === "text");
    texts[0].textContent = this.zaehler;
    texts[0].setAttribute("x", this.xposition);
    texts[0].addEventListener("click", () => {
      let clone = userinputfeld.content.cloneNode(true);
      let bla = clone.querySelectorAll("input")
      bla[0].value = this.zaehler;
      bla[0].addEventListener("change", () => {
        this.aendern("zaehler", bla[0].value)
        body.removeChild(bla[0])
      })
      body.appendChild(clone);

    });

    texts[1].textContent = this.nenner;
    texts[1].setAttribute("x", this.xposition);
    texts[1].addEventListener("click", () => {
      let clone = userinputfeld.content.cloneNode(true);
      let bla = clone.querySelectorAll("input");
      bla[0].value = this.nenner;
      bla[0].addEventListener("change", () => {
        this.aendern("nenner", bla[0].value);
        body.removeChild(bla[0]);
      });
      body.appendChild(clone);
    });

    this.gruppe.innerHTML="";
    Elements.forEach(node => this.gruppe.appendChild(node));

    this.alsBalkenDarstellen();

  }

  aendern(zuaenderndesElement, wert) {

    if(wert != null && !isNaN(wert)){
        eval("this." + zuaenderndesElement + "=" + wert);
        this.anzeigen();
    }else{
        alert("Ungültiger Wert, Nur Zahlen sind erlaubt")
    }
  }

  kuerzen(kuerzendeZahl){
    this.zaehler = this.zaehler/kuerzendeZahl;
    this.nenner = this.nenner/kuerzendeZahl;
    this.anzeigen();
  }

  alsBalkenDarstellen(){
    let Balken = [];
    if(this.nenner >= this.zaehler){
      for(let i=0; i < this.nenner; i++){
      let Element = document.createElementNS("http://www.w3.org/2000/svg", "rect")
      Element.setAttribute("width", 180/this.nenner);
      Element.setAttribute("height", 20);
      Element.setAttribute("x", (Element.getAttribute("width")*i)+10)
      Balken.push(Element)
    }
    for(let i=0; i < this.zaehler; i++){
      Balken[i].setAttribute("class", "used");
    }
    let group = document.createElementNS("http://www.w3.org/2000/svg", "g");
    Balken.forEach(rechteck => {
      group.appendChild(rechteck);
    }
    )
    this.gruppe.appendChild(group)
    }else{
      alert("feature zum Anzeigen von Werten > 1 noch im Aufbau");
    }

  }

}
class Bruch{
    constructor(zaehler, nenner){
        this.zaehler = zaehler;
        this.nenner = nenner;
    }

    anzeigen() {
        let clone = template.content;
        /*let textElements = clone.querySelectorAll("text");*/
    
        /*textElements.forEach(element => {
            element.addEventListener("click", (e) => {
                globalAendern(e.target);
            });
        });*/
        //svg.innerHTML = ""
        svg.appendChild(clone)
    }

    aendern(){
        console.log("debug");
    }
}


function globalAendern(element){

    console.log("Debug GloabelAenderung", element)
}
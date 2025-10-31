var xReihe = document.getElementById("x-werte");
let yReihe = document.getElementById("y-Reihe");
let svgpunkte = document.getElementById("Punkte");
let svgele = document.getElementById("svg1");
let svgviewbox = svgele.viewBox.baseVal;
let gitter = document.getElementById("gitterrect");
let xachse = document.getElementById("x-Achse");
let yachse = document.getElementById("y-Achse");
let funktionswerte = [];
var allepunktex = [];
var allepunktey = [];
var alleYWerte = [];
let zellen = [];
let minx;
let maxx;
let miny;
let maxy;
let Beschriftung = document.getElementById("Beschriftung");
let achsengruppe = document.getElementById("Achsengruppe");
let eingabefürinputs = document.getElementById("eingabe-werte"); //Mir fallen keine Namen mehr ein
function inputserstellen(){  //erstellt die inputs in der x-Reihe. Menge abhängig des Inputfeldes mit der id=eingabe-wert
    if(eingabefürinputs.value <= 2){
        eingabefürinputs.value  =  2;
    }
    xReihe.innerHTML = ""; //cleart die x-Reihe
    var th = document.createElement("th");
    th.innerText = "X-Wert"; //schreibt den th neu
    xReihe.appendChild(th);
    for(let i = 0; i<eingabefürinputs.value; i++){
        let input = document.createElement("input");
        input.type = "number";
        input.className = "eingabe";
        try{  //versucht die Werte aus der alten xReihe zu verwenden
            input.value = zellen[i].value; 
        }
        catch(err){  //wenn alte werte nicht existieren, wird einfach i verwendet
            input.value = Math.round(i - (eingabefürinputs.value-1)/2);  //Werte zu Beginn setzen. -1, weil 0 da ist. Die hälfte der Werte ist positiv, die andere negativ
        }
        let td = document.createElement("td");
        td.appendChild(input);
        xReihe.appendChild(td);
    }
    zellen = Array.from(document.getElementById("x-werte").getElementsByTagName("input"));
    //eventListener Hinzufügen
    for(let i=0; i<zellen.length; i++){
        zellen[i].addEventListener('change', () => {
            werteberechnung();
            zeichne();
        });
    }
    werteberechnung();
}
function funktionzuwertausrechnen(x){
    return 0.5 * (x ** 2) - 2 * x - 4;
}
function getallexwerte() {
    let allewertex = zellen.map(input => ({
            wert: Number(input.value)
        }));
        return allewertex; // gibt ein Array mit den Werten der Inputs zurück
}
function werteberechnung() {
        zellen = Array.from(document.getElementById("x-werte").getElementsByTagName("input")); // X-Werte-Inputs holen
        yReihe.innerHTML = "";
        var th = document.createElement("th");
        th.innerText = "Y-Wert";
        yReihe.appendChild(th);
        //Werte mit Referenz auf Inputs speichern
        allewertex = getallexwerte();
        //Sortieren nach dem X-Wert
        allewertex.sort((a, b) => a.wert - b.wert);
        //Sortierte Werte zurück in die Tabelle schreiben
        for (let i = 0; i < allewertex.length; i++) {
            zellen[i].value = allewertex[i].wert;
        }
        //Y-Werte berechnen und in die Tabelle schreiben
        for (let i = 0; i < allewertex.length; i++) {
            let xwert = allewertex[i].wert;
            let ywert = funktionzuwertausrechnen(xwert);
            alleYWerte[i] = ywert;
            let td = document.createElement('td');
            td.innerText = ywert;
            funktionswerte[i] = ywert;
            yReihe.appendChild(td);
        }
    }
function zeichne(){
    //alte Sachen clearen
    achsengruppe.innerHTML = "";
    Beschriftung.innerHTML = "";
    svgpunkte.innerHTML = "";
    var zellen = Array.from(document.getElementById("x-werte").getElementsByTagName("input"));
    //Punkte machen, setzen usw.
    for (i = 0; i < zellen.length; i++) {
        //Punkt erstellen
        let punkt = document.createElementNS("http://www.w3.org/2000/svg", "circle");
        //nötige variablen setzen und in Arrays speichern
        var xwert = Number(zellen[i].value);
        var ywert = Number(alleYWerte[i]);
        var punktx = xwert * 25;
        var punkty = -ywert * 25;
        allepunktex[i] = punktx;
        allepunktey[i] = punkty;
        //Attribute setzen und Punkt einfügen
        punkt.setAttribute("cx", punktx);
        punkt.setAttribute("cy", punkty);
        punkt.setAttribute("r", 3);
        punkt.setAttribute("class", "punkt");
        svgpunkte.appendChild(punkt);
    }
    //Min- und Max-Werte setzen
    minx = Math.min(...allepunktex);
    maxx = Math.max(...allepunktex);
    miny = Math.min(...allepunktey);
    maxy = Math.max(...allepunktey);
    let widths = Math.abs(minx) + Math.abs(maxx);
    let heights = Math.abs(miny) + Math.abs(maxy);
    //svg skalieren
    svgele.setAttribute("height", Math.abs(heights + 50));
    svgele.setAttribute("width", Math.abs(widths + 50));
    svgele.setAttribute("x", minx);
    svgele.setAttribute("y", maxy);
    //${} ist für dynamischen string. Wird ersetzt durch das Ergebnis der Rechnung dadrin
    svgele.setAttribute("viewBox", `${minx - 50} ${miny - 50} ${widths + 100} ${heights + 100}`);//
    //Gitterlinien machen
    gitter.setAttribute("x", minx - 50);
    gitter.setAttribute("y", miny - 100);
    gitter.setAttribute("width", widths + 200);
    gitter.setAttribute("height", heights + 200);
    //Achsen erschaffen
    let xAchse = document.createElementNS("http://www.w3.org/2000/svg", "line");
    let yAchse = document.createElementNS("http://www.w3.org/2000/svg", "line");
    //x-Achsenbeschriftung zu den Werten erstellen
    for (let i = 0; i < zellen.length; i++) {
        let text = document.createElementNS("http://www.w3.org/2000/svg", "text");
        let beschriftungx = Number(zellen[i].value);
        let positionx = allepunktex[i];
        if (beschriftungx == 0) {
            text.setAttribute("x", positionx + 10);
            text.setAttribute("y", 30);
            text.setAttribute("class", "textfeldx");
            text.innerHTML = beschriftungx;
            Beschriftung.appendChild(text);
        }else {
            text.setAttribute("x", positionx - 10);
            text.setAttribute("y", 30);
            text.setAttribute("class", "textfeldx");
            text.innerHTML = beschriftungx;
            Beschriftung.appendChild(text);
        }
    }
    //x-Achsenbeschriftung "x-Achse" hinzufügen
    let textbeschriftungx = document.createElementNS("http://www.w3.org/2000/svg", "text");
    textbeschriftungx.setAttribute("x", allepunktex[allepunktex.length - 1] - 80);
    textbeschriftungx.setAttribute("y", -20);
    textbeschriftungx.setAttribute("class", "beschriftung");
    textbeschriftungx.innerHTML = "x-Achse";
    Beschriftung.appendChild(textbeschriftungx);
    //y-Achsenbeschriftung zu den Werten erstellen
    for (let i = 0; i < zellen.length; i++) {
        let text = document.createElementNS("http://www.w3.org/2000/svg", "text");
        let beschriftungy = Number(alleYWerte[i]);
        let positiony = Number(allepunktey[i]);
        if (beschriftungy == 0) { }
        else {
            text.setAttribute("x", -35);
            text.setAttribute("y", positiony);
            text.setAttribute("class", "beschriftung");
            text.innerHTML = beschriftungy;
            Beschriftung.appendChild(text);
        }
    }
    //y-Achsenbeschriftung "y-Achse" hinzufügen
    let textbeschriftungy = document.createElementNS("http://www.w3.org/2000/svg", "text");
    textbeschriftungy.setAttribute("x", 20);
    textbeschriftungy.setAttribute("y", miny - 20);
    textbeschriftungy.setAttribute("class", "textfeldy");
    textbeschriftungy.innerHTML = "y-Achse";
    Beschriftung.appendChild(textbeschriftungy);
    // Achsen-Eigenschaften setzen
    xAchse.setAttribute("x1", minx - 50);
    xAchse.setAttribute("x2", maxx + 50);
    xAchse.setAttribute("y1", 0);
    xAchse.setAttribute("y2", 0);
    xAchse.setAttribute("class", "achsen");
    yAchse.setAttribute("x1", 0);
    yAchse.setAttribute("x2", 0);
    yAchse.setAttribute("y1", miny - 100);
    yAchse.setAttribute("y2", maxy + 100);
    yAchse.setAttribute("class", "achsen");
    // Achsen zum SVG hinzufügen
    achsengruppe.appendChild(xAchse);
    achsengruppe.appendChild(yAchse);
    zeichneKurve();
}
function zeichneKurve() {
    allewertex = getallexwerte();
    let imaginärepunktex = [];
    imaginärepunktex[0] = allewertex[0].wert;
    let imaginärepunkty = [];
    while (imaginärepunktex[imaginärepunktex.length - 1] < allewertex[allewertex.length - 1].wert) {
        let letzterWert = imaginärepunktex[imaginärepunktex.length - 1];
        let neuerX = letzterWert + 0.001;
        let neuerY = funktionzuwertausrechnen(neuerX);
    if (!isNaN(neuerY)) { // Nur gültige Werte hinzufügen
        imaginärepunktex.push(neuerX);
        imaginärepunkty.push(neuerY);
    } else {
        console.error(`Ungültiger Wert für x=${neuerX}: y=${neuerY}`);
        break; // Schleife abbrechen, falls ungültige Werte auftreten
    }
}
    const graphen = document.getElementById("graphen");
    graphen.innerHTML = "";
    // Generate path data using imaginäre Punkte
    let pathData = `M ${imaginärepunktex[0] * 25} ${-imaginärepunkty[0] * 25} `; // Startpunkt
    for (let i = 1; i < imaginärepunktex.length; i++) {
        if (!isNaN(imaginärepunkty[i])) { // Nur gültige Punkte verwenden
        pathData += `L ${imaginärepunktex[i] * 25} ${-imaginärepunkty[i] * 25} `;
    }
}
    // Create and append the path
    const pathEl = document.createElementNS("http://www.w3.org/2000/svg", "path");
    pathEl.setAttribute("d", pathData.trim());
    pathEl.setAttribute("stroke", "blue");
    pathEl.setAttribute("fill", "none");
    pathEl.setAttribute("stroke-width", "2");
    graphen.appendChild(pathEl);
}
eingabefürinputs.addEventListener('change', () => {
    inputserstellen();
    werteberechnung();
    zeichne();
});
inputserstellen();
werteberechnung();
zeichne();